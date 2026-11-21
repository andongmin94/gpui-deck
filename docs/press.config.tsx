import { createPress, type HeadTag } from "fumapress";
import { oramaSearchPlugin } from "fumapress/plugins/orama-search";
import { sitemapPlugin } from "fumapress/plugins/sitemap";
import { metaSchema, pageSchema } from "fumapress/schema";
import { defineDocs } from "fumadocs-mdx/macro";
import { findNeighbour } from "fumadocs-core/page-tree";

import { Home } from "@/components/home";
import { DocsShell } from "@/components/shell/docs-shell";
import { accents, accentStorageKey } from "@/lib/accent";
import { i18n, translations } from "@/lib/i18n";
import { SITE_DESCRIPTION, SITE_NAME, SITE_URL } from "@/lib/site";

const docs = defineDocs({
  dir: "content",
  docs: {
    async: true,
    schema: pageSchema,
  },
  meta: {
    schema: metaSchema,
  },
});

const app = createPress(docs.toFumadocsSource(), { i18n, mode: "force-static" }).plugins(
  oramaSearchPlugin(),
  sitemapPlugin(),
);

export default app.config({
  preset: [],
  site: {
    name: SITE_NAME,
    baseUrl: SITE_URL,
    hreflang: { zh: "zh-Hans" },
  },
  head({ page }): HeadTag[] {
    const icon: HeadTag = {
      tag: "link",
      attrs: { rel: "icon", href: "/deck.svg", type: "image/svg+xml" },
    };
    // Restore the accent before the browser chooses the first invoice preview image.
    const restoreAccent: HeadTag = {
      tag: "script",
      children: `try { const value = localStorage.getItem(${JSON.stringify(accentStorageKey)}); if (${JSON.stringify(accents)}.includes(value)) document.documentElement.dataset.accent = value; } catch {}`,
    };
    if (!page) {
      return [
        icon,
        restoreAccent,
        { tag: "meta", attrs: { property: "og:site_name", content: SITE_NAME } },
      ];
    }
    const title = page.slugs.length === 0 ? SITE_NAME : `${page.data.title} – ${SITE_NAME}`;
    const description = page.data.description ?? SITE_DESCRIPTION;
    return [
      icon,
      restoreAccent,
      { tag: "title", children: title },
      { tag: "meta", attrs: { name: "description", content: description } },
      { tag: "meta", attrs: { property: "og:title", content: title } },
      { tag: "meta", attrs: { property: "og:description", content: description } },
    ];
  },
  ui: {
    translations,
    renderPage(props) {
      if (props.page.slugs.length === 0) return <Home lang={props.lang} />;
      const { tree } = props.layoutProps;
      const { previous, next } = findNeighbour(tree, props.page.url);
      return (
        <DocsShell
          lang={props.lang}
          tree={tree}
          title={props.page.data.title}
          description={props.page.data.description}
          toc={props.toc}
          previous={previous && { name: previous.name, url: previous.url }}
          next={next && { name: next.name, url: next.url }}
        >
          {props.body}
        </DocsShell>
      );
    },
  },
});
