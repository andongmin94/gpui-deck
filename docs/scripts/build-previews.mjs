// Builds the live GPUI previews into public/preview: a host page and the
// wasm-bindgen output in a directory named by the content hash, which
// public/_headers caches as immutable. Cloudflare Workers Builds starts
// without Rust, so a missing toolchain is installed first (Linux only).
import { execFileSync, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  renameSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { homedir, tmpdir } from "node:os";
import { delimiter, join } from "node:path";
import { fileURLToPath } from "node:url";

const WASM_BINDGEN = "0.2.127";
const TARGET = "wasm32-unknown-unknown";
const root = new URL("../..", import.meta.url);
const cargoBin = join(homedir(), ".cargo", "bin");
const env = { ...process.env, PATH: `${cargoBin}${delimiter}${process.env.PATH}` };

function run(command, args, options = {}) {
  execFileSync(command, args, { cwd: root, env, stdio: "inherit", ...options });
}

function found(command, args = ["--version"]) {
  return spawnSync(command, args, { cwd: root, env, stdio: "ignore", shell: false }).status === 0;
}

if (!found("cargo")) {
  if (process.platform !== "linux") {
    throw new Error("Install Rust with rustup, then rerun the preview build.");
  }
  // The repository's rust-toolchain.toml selects the pinned toolchain on first use.
  run("sh", [
    "-c",
    "curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none",
  ]);
}
run("rustup", ["target", "add", TARGET]);

if (
  !found("wasm-bindgen") ||
  !execFileSync("wasm-bindgen", ["--version"], { env }).toString().includes(WASM_BINDGEN)
) {
  if (process.platform !== "linux") {
    throw new Error(`Install wasm-bindgen-cli ${WASM_BINDGEN}, then rerun the preview build.`);
  }
  const archive = `wasm-bindgen-${WASM_BINDGEN}-x86_64-unknown-linux-musl`;
  const dir = join(tmpdir(), "wasm-bindgen");
  mkdirSync(dir, { recursive: true });
  run("sh", [
    "-c",
    `curl -sSfL https://github.com/wasm-bindgen/wasm-bindgen/releases/download/${WASM_BINDGEN}/${archive}.tar.gz | tar -xz -C "${dir}" && mkdir -p "${cargoBin}" && cp "${dir}/${archive}/wasm-bindgen" "${cargoBin}/"`,
  ]);
}

run("cargo", [
  "build",
  "-p",
  "gpui-deck-web-preview",
  "--target",
  TARGET,
  "--profile",
  "web-preview",
  "--locked",
]);
const wasm = join("target", TARGET, "web-preview", "gpui_deck_web_preview.wasm");
if (!existsSync(new URL(wasm.replaceAll("\\", "/"), root))) throw new Error(`missing ${wasm}`);

const out = new URL("../public/preview/", import.meta.url);
rmSync(out, { recursive: true, force: true });
const staging = new URL("staging/", out);
run("wasm-bindgen", [
  "--target",
  "web",
  "--no-typescript",
  "--out-dir",
  fileURLToPath(staging),
  wasm,
]);
const hash = createHash("sha256")
  .update(readFileSync(new URL("gpui_deck_web_preview_bg.wasm", staging)))
  .update(readFileSync(new URL("gpui_deck_web_preview.js", staging)))
  .digest("hex")
  .slice(0, 12);
renameSync(staging, new URL(`${hash}/`, out));
copyFileSync(
  new URL("crates/base/examples/assets/fonts/LICENSE", root),
  new URL("Pretendard-LICENSE.txt", out),
);
writeFileSync(new URL("index.html", out), hostPage(`/preview/${hash}/gpui_deck_web_preview.js`));

/**
 * The frame page. A same-origin frame follows the theme of the docs page and
 * reloads when it changes, so the docs page can render the frame before it
 * hydrates. A spinner shows until GPUI adds its canvas.
 */
function hostPage(module) {
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>GPUI Deck preview</title>
    <link rel="license" title="Embedded Pretendard fonts" href="/preview/Pretendard-LICENSE.txt" />
    <link rel="modulepreload" href="${module}" />
    <style>
      html, body { margin: 0; height: 100%; overflow: hidden; }
      body { background: #ffffff; color: #737373; }
      body.dark { background: #0a0a0a; color: #a1a1a1; }
      .status { position: fixed; inset: 0; display: grid; place-items: center; font: 14px system-ui, sans-serif; }
      .spinner { width: 20px; height: 20px; border: 2px solid currentColor; border-right-color: transparent; border-radius: 50%; animation: spin 0.8s linear infinite; }
      @keyframes spin { to { transform: rotate(360deg); } }
    </style>
  </head>
  <body>
    <div class="status" role="status" aria-label="Loading preview"><div class="spinner"></div></div>
    <script type="module">
      const params = new URLSearchParams(location.search);
      const host = parent !== window ? parent.document.documentElement : null;
      const themeOf = (root) =>
        root?.classList.contains("dark") ||
        (!root && matchMedia("(prefers-color-scheme: dark)").matches)
          ? "dark"
          : "light";
      const theme = params.get("theme") ?? themeOf(host);
      if (params.get("theme") !== theme) {
        params.set("theme", theme);
        history.replaceState(null, "", "?" + params);
      }
      document.body.classList.toggle("dark", theme === "dark");
      if (host) {
        new MutationObserver(() => {
          const next = themeOf(host);
          if (next !== theme) {
            params.set("theme", next);
            location.replace("?" + params);
          }
        }).observe(host, { attributes: true, attributeFilter: ["class"] });
      }
      const status = document.querySelector(".status");
      if (!("gpu" in navigator)) {
        status.textContent = "This preview needs a browser with WebGPU.";
      } else {
        new MutationObserver((_, observer) => {
          if (document.querySelector("canvas")) {
            status.remove();
            observer.disconnect();
          }
        }).observe(document.body, { childList: true, subtree: true });
        const { default: init } = await import("${module}");
        await init();
      }
    </script>
  </body>
</html>
`;
}
