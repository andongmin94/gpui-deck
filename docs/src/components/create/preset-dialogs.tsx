import { Dialog } from "@base-ui/react/dialog";
import { Link } from "fumapress/client";
import { Check, Code2, Copy, FolderOpen, X } from "lucide-react";
import { lazy, type ReactNode, Suspense, useState } from "react";

import { type Preset, encodePreset, parsePreset, rustPreset } from "@/lib/create-preset";
import { blockCopy } from "../blocks/block-copy";
import { InstallCommand } from "../blocks/install-command";
import type { CreateCopy } from "./copy";
import iconsSource from "../../../../crates/cli/presets/create/icons.rs?url&no-inline";
import moduleSource from "../../../../crates/cli/presets/create/mod.rs?url&no-inline";
import showcaseSource from "../../../../crates/cli/presets/create/showcase.rs?url&no-inline";
import styleSource from "../../../../crates/cli/presets/create/style.rs?url&no-inline";
import gallerySource from "../../../../crates/cli/templates/gallery.rs?url&no-inline";

const primary =
  "inline-flex min-h-10 items-center justify-center gap-2 rounded-lg bg-fd-primary px-4 text-sm font-semibold text-fd-primary-foreground hover:opacity-85 disabled:opacity-60";
const pre = "overflow-x-auto rounded-lg border bg-fd-secondary p-3 font-mono text-xs leading-6";

function Modal({ title, close, children }: { title: string; close: string; children: ReactNode }) {
  return (
    <Dialog.Portal>
      <Dialog.Backdrop className="fixed inset-0 z-50 bg-black/45" />
      <Dialog.Popup className="fixed top-1/2 left-1/2 z-50 max-h-[90dvh] w-[min(36rem,calc(100%-2rem))] -translate-x-1/2 -translate-y-1/2 overflow-y-auto rounded-2xl border bg-fd-background p-5 text-fd-foreground shadow-xl outline-none sm:p-6">
        <div className="mb-5 flex items-center justify-between gap-3">
          <Dialog.Title className="text-xl font-semibold tracking-tight">{title}</Dialog.Title>
          <Dialog.Close
            aria-label={close}
            className="inline-flex size-9 items-center justify-center rounded-md hover:bg-fd-secondary"
          >
            <X aria-hidden className="size-4" />
          </Dialog.Close>
        </div>
        {children}
      </Dialog.Popup>
    </Dialog.Portal>
  );
}

export function OpenPreset({ t, onApply }: { t: CreateCopy; onApply: (preset: Preset) => void }) {
  const [open, setOpen] = useState(false);
  const [value, setValue] = useState("");
  const [invalid, setInvalid] = useState(false);
  return (
    <Dialog.Root
      open={open}
      onOpenChange={(next) => {
        setOpen(next);
        if (next) {
          setValue("");
          setInvalid(false);
        }
      }}
    >
      <Dialog.Trigger className="inline-flex min-h-9 min-w-0 items-center justify-center gap-2 rounded-[7px] border border-white/15 bg-white/[0.025] px-2 text-sm font-medium hover:bg-white/10 focus-visible:outline-2 focus-visible:outline-white md:col-span-full">
        <FolderOpen aria-hidden className="size-4 shrink-0" />
        <span className="truncate">{t.open}</span>
      </Dialog.Trigger>
      <Modal title={t.open} close={t.close}>
        <form
          onSubmit={(event) => {
            event.preventDefault();
            const preset = parsePreset(value.trim());
            if (!preset) {
              setInvalid(true);
              return;
            }
            onApply(preset);
            setOpen(false);
          }}
        >
          <label className="grid gap-2 text-sm font-medium">
            {t.presetInput}
            <input
              type="text"
              autoComplete="off"
              spellCheck={false}
              value={value}
              onChange={(event) => {
                setValue(event.target.value);
                setInvalid(false);
              }}
              aria-invalid={invalid}
              aria-describedby={invalid ? "preset-input-error" : undefined}
              className="h-11 min-w-0 rounded-lg border bg-fd-background px-3 font-mono text-sm"
            />
          </label>
          {invalid && (
            <p id="preset-input-error" role="alert" className="mt-2 text-sm text-fd-error">
              {t.invalid}
            </p>
          )}
          <div className="mt-4 flex justify-end">
            <button type="submit" className={primary}>
              {t.apply}
            </button>
          </div>
        </form>
      </Modal>
    </Dialog.Root>
  );
}

function RustSource({ t, source, name }: { t: CreateCopy; source: string; name: string }) {
  const [status, setStatus] = useState<"idle" | "copied" | "manual">("idle");
  return (
    <div className="relative">
      <pre
        aria-label={name}
        // oxlint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- Let keyboard users scroll the installed source in both directions.
        tabIndex={0}
        className={`${pre} max-h-[35svh] overflow-auto pe-12`}
      >
        <code>{source}</code>
      </pre>
      <button
        type="button"
        aria-label={t.copyCode}
        title={t.copyCode}
        className="absolute end-1 top-1 inline-flex size-8 items-center justify-center rounded-md border bg-fd-background hover:bg-fd-secondary"
        onClick={async () => {
          try {
            await navigator.clipboard.writeText(source);
            setStatus("copied");
          } catch {
            setStatus("manual");
          }
        }}
      >
        {status === "copied" ? (
          <Check aria-hidden className="size-4" />
        ) : (
          <Copy aria-hidden className="size-4" />
        )}
      </button>
      <output className={status === "manual" ? "text-xs text-fd-muted-foreground" : "sr-only"}>
        {status === "copied" ? t.copied : status === "manual" ? t.manual : ""}
      </output>
    </div>
  );
}

// Vite emits the actual Rust files as assets. Fetch just the selected file as text.
const sourceFiles = {
  "style.rs": styleSource,
  "showcase.rs": showcaseSource,
  "icons.rs": iconsSource,
  "mod.rs": moduleSource,
  "gallery.rs": gallerySource,
};
const rustViews = Object.fromEntries(
  Object.entries(sourceFiles).map(([name, url]) => {
    return [
      name,
      lazy(async () => {
        const response = await fetch(url);
        if (!response.ok) throw new Error(`Failed to load ${name} (${response.status})`);
        const source = await response.text();
        return {
          default: ({ t }: { t: CreateCopy }) => <RustSource t={t} name={name} source={source} />,
        };
      }),
    ];
  }),
);

function SourceFiles({
  lang,
  t,
  preset,
  dark,
}: {
  lang: string;
  t: CreateCopy;
  preset: Preset;
  dark: boolean;
}) {
  const [file, setFile] = useState("selected.rs");
  const View = rustViews[file];
  const labels = blockCopy(lang);
  return (
    <>
      <section>
        <div className="mb-2 flex items-center justify-between gap-2">
          <h3 className="text-sm font-semibold">{t.rust}</h3>
          <label>
            <span className="sr-only">{labels.file}</span>
            <select
              value={file}
              onChange={(e) => setFile(e.target.value)}
              className="min-h-8 rounded-md border bg-fd-background px-2 font-mono text-xs"
            >
              {["selected.rs", ...Object.keys(rustViews)].map((name) => (
                <option key={name} value={name}>
                  {name === "gallery.rs" ? t.gallerySource : name}
                </option>
              ))}
            </select>
          </label>
        </div>
        {file === "gallery.rs" && (
          <p className="mb-2 text-sm leading-relaxed text-fd-muted-foreground">{t.gallerySetup}</p>
        )}
        {file === "selected.rs" ? (
          <RustSource t={t} name={file} source={rustPreset(preset)} />
        ) : (
          <Suspense fallback={<output className="text-sm">{labels.loading}</output>}>
            <View t={t} />
          </Suspense>
        )}
      </section>
      <section>
        <h3 className="mb-2 text-sm font-semibold">{t.use}</h3>
        <pre className={`${pre} whitespace-pre-wrap break-words`}>
          <code>
            {file === "gallery.rs"
              ? `let view = gallery::open(${dark}, window, cx);`
              : `let view = ui::preset::showcase(${dark}, window, cx);`}
          </code>
        </pre>
      </section>
    </>
  );
}

export function GetCode({
  preset,
  dark,
  lang,
  t,
  ready,
}: {
  preset: Preset;
  dark: boolean;
  lang: string;
  t: CreateCopy;
  ready: boolean;
}) {
  const code = encodePreset(preset);
  return (
    <Dialog.Root>
      <Dialog.Trigger
        disabled={!ready}
        className="col-start-3 row-start-2 inline-flex min-h-9 items-center justify-center gap-1.5 rounded-[7px] bg-neutral-100 px-3 text-sm font-semibold text-neutral-950 hover:bg-white focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white disabled:opacity-60 md:col-start-2 md:row-start-1"
      >
        <Code2 aria-hidden className="size-4 shrink-0" />
        {t.getCode}
      </Dialog.Trigger>
      <Modal title={t.getCode} close={t.close}>
        <div className="grid gap-5">
          <section>
            <h3 className="mb-2 text-sm font-semibold">{t.newApp}</h3>
            <InstallCommand
              lang={lang}
              command={`gpui-deck init my-app --preset ${code}${dark ? " --dark" : ""}`}
            />
            <pre className={`${pre} mt-2`}>
              <code>{"cd my-app\ncargo run"}</code>
            </pre>
            <details className="mt-2 text-sm">
              <summary className="cursor-pointer py-1 text-fd-muted-foreground">{t.cli}</summary>
              <InstallCommand
                lang={lang}
                command="cargo install gpui-deck-cli --version 0.0.4 --locked"
              />
            </details>
            <details className="mt-2 text-sm">
              <summary className="cursor-pointer py-1 text-fd-muted-foreground">
                {t.existingApp}
              </summary>
              <InstallCommand lang={lang} command={`gpui-deck add --preset ${code}`} />
            </details>
            <Link
              href={`/${lang}/docs/installation`}
              className="mt-2 inline-flex text-sm text-fd-muted-foreground underline underline-offset-4 hover:text-fd-foreground"
            >
              {blockCopy(lang).setup}
            </Link>
          </section>
          <SourceFiles key={code} lang={lang} t={t} preset={preset} dark={dark} />
        </div>
      </Modal>
    </Dialog.Root>
  );
}
