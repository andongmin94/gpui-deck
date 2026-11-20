import { source } from '@/lib/source';
import { DocsBody, DocsDescription, DocsPage, DocsTitle, EditOnGitHub } from 'fumadocs-ui/layouts/docs/page';
import { notFound } from 'next/navigation';
import { createRelativeLink } from 'fumadocs-ui/mdx';
import type { Metadata } from 'next';
import { getMDXComponents } from '@/components/mdx';
import { gitConfig } from '@/lib/shared';
import { i18n } from '@/lib/i18n';
import { Callout } from 'fumadocs-ui/components/callout';
import Link from 'next/link';

export default async function Page(props: PageProps<'/[lang]/docs/[[...slug]]'>) {
  const { lang, slug } = await props.params;
  const page = source.getPage(slug, lang);
  if (!page) notFound();

  const MDX = page.data.body;
  const original = lang === i18n.defaultLanguage ? undefined : source.getPage(slug, i18n.defaultLanguage);
  // An untranslated page falls back to the English text unchanged.
  const translated = original !== undefined && original.data.description !== page.data.description;
  return (
    <DocsPage toc={page.data.toc} full={page.data.full}>
      <DocsTitle>{page.data.title}</DocsTitle>
      <DocsDescription>{page.data.description}</DocsDescription>
      {original && (
        <Callout type={translated ? 'info' : 'warn'}>
          {translated
            ? '이 페이지는 영어 원문을 번역한 것으로 오류가 있을 수 있습니다. '
            : '이 페이지는 아직 번역되지 않아 영어 원문을 보여 줍니다. '}
          <Link href={original.url}>영어 원문 보기</Link>
        </Callout>
      )}
      <DocsBody>
        <MDX components={getMDXComponents({ a: createRelativeLink(source, page) })} />
      </DocsBody>
      <EditOnGitHub
        href={`https://github.com/${gitConfig.user}/${gitConfig.repo}/blob/${gitConfig.branch}/docs/content/docs/${page.path}`}
      />
    </DocsPage>
  );
}

export function generateStaticParams() {
  return source.generateParams();
}

export async function generateMetadata(
  props: PageProps<'/[lang]/docs/[[...slug]]'>,
): Promise<Metadata> {
  const { lang, slug } = await props.params;
  const page = source.getPage(slug, lang);
  if (!page) notFound();
  return { title: page.data.title, description: page.data.description };
}
