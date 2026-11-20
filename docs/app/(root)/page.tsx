import { basePath } from '@/lib/shared';

// Static hosting cannot negotiate the language, so pick it in the browser.
const script = `location.replace('${basePath}/' + (navigator.language.startsWith('ko') ? 'ko' : 'en') + '/');`;

export default function RootPage() {
  return (
    <>
      <script dangerouslySetInnerHTML={{ __html: script }} />
      <noscript>
        <a href={`${basePath}/en/`}>English</a> · <a href={`${basePath}/ko/`}>한국어</a>
      </noscript>
    </>
  );
}
