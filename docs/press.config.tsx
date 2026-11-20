import { createPress, type HeadTag } from "fumapress";
import { oramaSearchPlugin } from "fumapress/plugins/orama-search";
import { sitemapPlugin } from "fumapress/plugins/sitemap";
import { metaSchema, pageSchema } from "fumapress/schema";
import { defineDocs } from "fumadocs-mdx/macro";
import { HomeLayout } from "fumadocs-ui/layouts/home";

import { Home } from "@/components/home";
import { TranslationNotice } from "@/components/translation-notice";
import { i18n, translations } from "@/lib/i18n";
import { GIT, SITE_DESCRIPTION, SITE_NAME, SITE_URL } from "@/lib/site";

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
  {
    name: "translation-notice",
    slots({ lang, page }) {
      if (!page || lang === i18n.defaultLanguage || page.slugs.length === 0) return;
      return [
        {
          name: "translation-notice",
          region: "body-wrapper",
          component: TranslationNotice,
          props: {
            original: page.url.replace(`/${lang}/`, `/${i18n.defaultLanguage}/`),
            translated: page.locale === lang,
          },
        },
      ];
    },
  },
);

export default app.config({
  preset: [],
  site: {
    name: SITE_NAME,
    baseUrl: SITE_URL,
    git: { ...GIT, rootDir: "docs" },
  },
  head({ page }): HeadTag[] {
    if (!page) {
      return [{ tag: "meta", attrs: { property: "og:site_name", content: SITE_NAME } }];
    }
    const title = page.slugs.length === 0 ? SITE_NAME : `${page.data.title} – ${SITE_NAME}`;
    const description = page.data.description ?? SITE_DESCRIPTION;
    return [
      { tag: "title", children: title },
      { tag: "meta", attrs: { name: "description", content: description } },
      { tag: "meta", attrs: { property: "og:title", content: title } },
      { tag: "meta", attrs: { property: "og:description", content: description } },
    ];
  },
  ui: {
    translations,
    layoutProps({ lang }) {
      const ko = lang === "ko";
      return {
        i18n: true,
        nav: {
          title: (
            <span className="inline-flex items-center gap-2 font-semibold">
              <span className="size-5 rounded-md bg-fd-foreground" aria-hidden />
              {SITE_NAME}
            </span>
          ),
          url: `/${lang}`,
        },
        links: [
          { text: ko ? "문서" : "Docs", url: `/${lang}/docs`, active: "nested-url" },
          {
            text: ko ? "컴포넌트" : "Components",
            url: `/${lang}/docs/components`,
            active: "nested-url",
          },
        ],
        githubUrl: `https://github.com/${GIT.user}/${GIT.repo}`,
      };
    },
    renderPage(props) {
      if (props.page.slugs.length === 0) {
        const { tree: _tree, ...layoutProps } = props.layoutProps;
        return (
          <HomeLayout {...layoutProps}>
            <Home lang={props.lang} />
          </HomeLayout>
        );
      }
      return props.renderDefault();
    },
  },
});
