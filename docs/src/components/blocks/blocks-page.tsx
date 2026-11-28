import { Link } from "fumapress/client";
import { ArrowLeft } from "lucide-react";
import { registryBlockDescription } from "@/lib/block-descriptions";

import { CatalogLayout } from "../catalog-layout";
import { itemTitle, registries } from "../directory/registries";
import { BlockInstall } from "./block-install";
import { FilteredBlocks, type Screen } from "./filtered-blocks";
import { type Block, blockCopy, purposes } from "./block-copy";
import { BlockViewer } from "./block-viewer";

type PageInfo = { title: string; description?: string };
type GalleryEntry = PageInfo & { block: Block };

export function BlocksGallery({
  lang,
  page,
  entries,
}: {
  lang: string;
  page: PageInfo;
  entries: GalleryEntry[];
}) {
  const t = blockCopy(lang);
  const screens: Screen[] = entries.map(({ block, title, description }) => ({
    block,
    title,
    description,
    components: block.components,
    purpose: block.purpose,
    style: "base",
    href: `/${lang}/blocks/${block.name}`,
  }));
  const styles: Record<string, string> = { base: "Base" };
  for (const registry of registries) {
    styles[registry.slug] = registry.name;
    for (const item of registry.items) {
      if (item.kind !== "template" && item.kind !== "block") continue;
      const purpose = purposes.find((value) => value === item.purpose);
      if (!purpose) throw new Error(`Missing block purpose for ${item.name}`);
      screens.push({
        block: item,
        title: itemTitle(item.slug, lang),
        description: registryBlockDescription(item.slug, lang),
        purpose,
        style: registry.slug,
        href: `/${lang}/directory/${registry.slug}#${item.slug}`,
      });
    }
  }
  return (
    <CatalogLayout lang={lang}>
      <header className="mx-auto flex max-w-3xl flex-col items-center py-8 text-center sm:py-16">
        <h1 className="text-4xl font-semibold tracking-[-0.045em] text-balance sm:text-5xl">
          {t.galleryTitle}
        </h1>
        <p className="mt-4 max-w-xl text-base leading-relaxed text-balance text-fd-muted-foreground sm:text-lg">
          {page.description}
        </p>
      </header>
      <FilteredBlocks lang={lang} screens={screens} styles={styles} />
    </CatalogLayout>
  );
}

export function BlockDetail({ lang, page, block }: { lang: string; page: PageInfo; block: Block }) {
  const t = blockCopy(lang);
  return (
    <CatalogLayout lang={lang}>
      <div className="py-7 sm:pt-11 sm:pb-7">
        <Link
          href={`/${lang}/blocks`}
          className="inline-flex items-center gap-1.5 rounded text-sm font-medium text-fd-muted-foreground hover:text-fd-foreground"
        >
          <ArrowLeft aria-hidden className="size-4" />
          {t.all}
        </Link>
        <h1 className="mt-5 text-3xl font-semibold tracking-[-0.045em] sm:text-5xl">
          {page.title}
        </h1>
        <p className="mt-3 max-w-2xl text-base leading-relaxed text-fd-muted-foreground sm:text-lg">
          {page.description}
        </p>
      </div>
      <BlockInstall lang={lang} block={block} />
      <BlockViewer key={block.name} lang={lang} title={page.title} block={block} />
    </CatalogLayout>
  );
}
