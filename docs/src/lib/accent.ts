export const accents = ["neutral", "blue", "green", "violet", "rose"] as const;
export type Accent = (typeof accents)[number];
export const accentStorageKey = "gpui-design-accent";
