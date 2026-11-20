import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import { appName, gitConfig } from './shared';

export function baseOptions(lang: string): BaseLayoutProps {
  const ko = lang === 'ko';
  return {
    i18n: true,
    nav: {
      title: (
        <span className="inline-flex items-center gap-2 font-semibold">
          <span className="size-5 rounded-md bg-fd-foreground" aria-hidden />
          {appName}
        </span>
      ),
      url: `/${lang}`,
    },
    links: [
      { text: ko ? '문서' : 'Docs', url: `/${lang}/docs`, active: 'nested-url' },
      { text: ko ? '컴포넌트' : 'Components', url: `/${lang}/docs/components`, active: 'nested-url' },
    ],
    githubUrl: `https://github.com/${gitConfig.user}/${gitConfig.repo}`,
  };
}
