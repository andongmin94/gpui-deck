import { Inter } from 'next/font/google';
import type { Metadata } from 'next';
import { Provider } from '@/components/provider';
import { i18n } from '@/lib/i18n';
import { i18nUI } from '@/lib/translations';
import '../global.css';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: { template: '%s – GPUI Design', default: 'GPUI Design' },
  description: 'Source-owned, headless UI components for Rust and GPUI.',
};

export function generateStaticParams() {
  return i18n.languages.map((lang) => ({ lang }));
}

export default async function Layout({ children, params }: LayoutProps<'/[lang]'>) {
  const { lang } = await params;
  return (
    <html lang={lang} className={inter.className} suppressHydrationWarning>
      <body className="flex flex-col min-h-screen">
        <Provider i18n={i18nUI.provider(lang)}>{children}</Provider>
      </body>
    </html>
  );
}
