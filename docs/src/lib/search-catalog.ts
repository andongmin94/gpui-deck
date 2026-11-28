import type { Block } from "@/components/blocks/block-copy";
import { chartCopy, chartTypes } from "@/components/charts/charts";
import { optionLabel } from "@/components/create/copy";
import { itemTitle, registries } from "@/components/directory/registries";
import { copyFor } from "@/components/shell/copy";
import { defaultPreset, encodePreset, options } from "./create-preset";

export type CatalogEntry = {
  title: string;
  url: string;
  breadcrumbs: string[];
  aliases: string[];
};

export function searchCatalog(
  lang: string,
  baseBlocks: { block: Block; title: string; english: string }[],
): CatalogEntry[] {
  const t = copyFor(lang);
  const charts = chartCopy(lang);
  const presetAliases = (title: string) => [
    "preset",
    [title, t.preset].join(lang === "ja" || lang === "zh" ? "" : " "),
  ];
  const entries: CatalogEntry[] = baseBlocks.map(({ block, title, english }) => ({
    title,
    url: `/${lang}/blocks/${block.name}`,
    breadcrumbs: [t.blocks, "Base"],
    aliases: [english, block.name],
  }));

  for (const registry of registries) {
    for (const item of registry.items) {
      const type = chartTypes.find((value) => value === item.chartType);
      const palette = item.kind === "theme" || item.kind === "base";
      if (!type && !palette && item.kind !== "block" && item.kind !== "template") continue;
      const title = (language: string) =>
        item.slug === "chart"
          ? chartCopy(language).examples[registry.slug as keyof typeof charts.examples]
          : itemTitle(item.slug, language);
      entries.push({
        title: title(lang),
        url: `/${lang}/directory/${registry.slug}#${item.slug}`,
        breadcrumbs: [palette ? t.preset : type ? t.charts : t.blocks, registry.name],
        aliases: [
          title("en"),
          item.slug,
          item.name,
          ...(palette ? presetAliases(title(lang)) : []),
        ],
      });
    }
  }
  for (const type of chartTypes) {
    entries.push({
      title: charts.types[type],
      url: `/${lang}/charts?type=${type}`,
      breadcrumbs: [t.charts],
      aliases: [type, chartCopy("en").types[type]],
    });
  }
  for (const style of options.style) {
    entries.push({
      title: optionLabel(style, lang),
      url: `/${lang}/create?preset=${encodePreset({ ...defaultPreset, style: style.value })}`,
      breadcrumbs: [t.create, t.preset],
      aliases: [style.value, style.label, ...presetAliases(optionLabel(style, lang))],
    });
  }
  return entries;
}
