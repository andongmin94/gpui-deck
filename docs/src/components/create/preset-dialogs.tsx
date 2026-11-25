import { Dialog } from "@base-ui/react/dialog";
import { Link } from "fumapress/client";
import { Check, Code2, Copy, FolderOpen, X } from "lucide-react";
import { lazy, type ReactNode, Suspense, useState } from "react";

import { type Preset, encodePreset, parsePreset, rustPreset } from "@/lib/create-preset";
import { blockCopy } from "../blocks/block-copy";
import { InstallCommand } from "../blocks/install-command";
import type { CreateCopy } from "./copy";

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
      <Dialog.Trigger className="inline-flex min-h-10 flex-1 items-center justify-center gap-2 rounded-lg border border-white/20 px-2 text-sm font-medium hover:bg-white/10">
        <FolderOpen aria-hidden className="size-4" />
        {t.open}
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
  "style.rs": new URL("../../../../crates/cli/presets/create/style.rs?no-inline", import.meta.url),
  "showcase.rs": new URL(
    "../../../../crates/cli/presets/create/showcase.rs?no-inline",
    import.meta.url,
  ),
  "icons.rs": new URL("../../../../crates/cli/presets/create/icons.rs?no-inline", import.meta.url),
  "mod.rs": new URL("../../../../crates/cli/presets/create/mod.rs?no-inline", import.meta.url),
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

function SourceFiles({ lang, t, preset }: { lang: string; t: CreateCopy; preset: Preset }) {
  const [file, setFile] = useState("selected.rs");
  const View = rustViews[file];
  const labels = blockCopy(lang);
  return (
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
              <option key={name}>{name}</option>
            ))}
          </select>
        </label>
      </div>
      {file === "selected.rs" ? (
        <RustSource t={t} name={file} source={rustPreset(preset)} />
      ) : (
        <Suspense fallback={<output className="text-sm">{labels.loading}</output>}>
          <View t={t} />
        </Suspense>
      )}
    </section>
  );
}

export function GetCode({
  preset,
  lang,
  t,
  ready,
}: {
  preset: Preset;
  lang: string;
  t: CreateCopy;
  ready: boolean;
}) {
  const code = encodePreset(preset);
  return (
    <Dialog.Root>
      <Dialog.Trigger disabled={!ready} className={primary}>
        <Code2 aria-hidden className="size-4" />
        {t.getCode}
      </Dialog.Trigger>
      <Modal title={t.getCode} close={t.close}>
        <div className="grid gap-5">
          <section>
            <h3 className="mb-2 text-sm font-semibold">{t.install}</h3>
            <InstallCommand lang={lang} command={`gpui-deck add --preset ${code}`} />
            <details className="mt-2 text-sm">
              <summary className="cursor-pointer py-1 text-fd-muted-foreground">{t.cli}</summary>
              <InstallCommand
                lang={lang}
                command="cargo install gpui-deck-cli --version 0.0.3 --locked"
              />
            </details>
            <Link
              href={`/${lang}/docs/installation`}
              className="mt-2 inline-flex text-sm text-fd-muted-foreground underline underline-offset-4 hover:text-fd-foreground"
            >
              {blockCopy(lang).setup}
            </Link>
          </section>
          <SourceFiles key={code} lang={lang} t={t} preset={preset} />
          <section>
            <h3 className="mb-2 text-sm font-semibold">{t.use}</h3>
            <pre className={`${pre} whitespace-pre-wrap break-words`}>
              <code>let view = ui::preset::showcase(false, window, cx);</code>
            </pre>
          </section>
        </div>
      </Modal>
    </Dialog.Root>
  );
}
