// Builds the live GPUI previews into public/preview. Cloudflare Workers Builds
// starts without Rust, so a missing toolchain is installed first (Linux only).
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import { homedir, tmpdir } from "node:os";
import { delimiter, join } from "node:path";

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
  "gpui-design-web-preview",
  "--target",
  TARGET,
  "--release",
  "--locked",
]);
const wasm = join("target", TARGET, "release", "gpui_design_web_preview.wasm");
if (!existsSync(new URL(wasm.replaceAll("\\", "/"), root))) throw new Error(`missing ${wasm}`);
run("wasm-bindgen", [
  "--target",
  "web",
  "--no-typescript",
  "--out-dir",
  join("docs", "public", "preview"),
  wasm,
]);
run(process.execPath, [join("docs", "scripts", "preview-html.mjs")]);
