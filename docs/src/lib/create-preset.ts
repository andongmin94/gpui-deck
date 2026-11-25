import catalog from "../../../crates/cli/presets/create/options.json";

export const axes = [
  "style",
  "base",
  "accent",
  "chart",
  "heading",
  "font",
  "icons",
  "radius",
] as const;
export type Axis = (typeof axes)[number];
export type Preset = Record<Axis, string>;
export const options = catalog;

const fromValues = (values: string[]) =>
  Object.fromEntries(axes.map((axis, index) => [axis, values[index]])) as Preset;

export const defaultPreset = fromValues(axes.map((axis) => options[axis][0].value));

export function encodePreset(preset: Preset): string {
  return ["v1", ...axes.map((axis) => preset[axis])].join(".");
}

export function parsePreset(input: string): Preset | null {
  let code = input.trim();
  if (code.startsWith("/") || /^https?:\/\//i.test(code)) {
    try {
      const url = new URL(code, "https://gpui-deck.andongmin.com");
      if (!/\/create\/?$/.test(url.pathname)) return null;
      code = url.searchParams.get("preset") ?? "";
    } catch {
      return null;
    }
  }
  const [version, ...values] = code.split(".");
  if (version !== "v1" || values.length !== axes.length) return null;
  if (!axes.every((axis, index) => options[axis].some((option) => option.value === values[index])))
    return null;
  return fromValues(values);
}

export function shufflePreset(): Preset {
  return fromValues(
    axes.map((axis) => {
      const choices = options[axis];
      return choices[Math.floor(Math.random() * choices.length)].value;
    }),
  );
}

export function rustPreset(preset: Preset): string {
  return `pub const CODE: &str = "${encodePreset(preset)}";\n`;
}
