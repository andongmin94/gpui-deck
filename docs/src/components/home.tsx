import { Link } from "fumapress/client";

import { ComponentPreview } from "./preview";

const copy = {
  en: {
    badge: "Headless components for Rust and GPUI",
    title: "Build your component library for GPUI.",
    lead: "Accessible behavior, focus and state in the library. Your own source, styles and layout in your application. Copy recipes in, then make them yours.",
    start: "Get Started",
    browse: "Browse Components",
    features: [
      [
        "Headless by design",
        "Controls own interaction, focus and accessibility semantics. Presentation stays in your code.",
      ],
      [
        "Source you own",
        "A local CLI copies presentation recipes, their dependencies and light/dark tokens into your app.",
      ],
      [
        "Native first",
        "Built on GPUI for desktop windows, with the same components running on the web for these previews.",
      ],
    ],
  },
  ko: {
    badge: "Rust와 GPUI를 위한 헤드리스 컴포넌트",
    title: "GPUI용 컴포넌트 라이브러리를 직접 만드세요.",
    lead: "접근성 있는 동작, 포커스, 상태는 라이브러리가 맡고 소스, 스타일, 레이아웃은 애플리케이션이 소유합니다. 레시피를 복사해 와서 내 것으로 바꾸세요.",
    start: "시작하기",
    browse: "컴포넌트 보기",
    features: [
      [
        "처음부터 헤드리스",
        "컨트롤은 상호작용, 포커스, 접근성 의미만 소유합니다. 화면 표현은 여러분의 코드에 남습니다.",
      ],
      [
        "내가 소유하는 소스",
        "로컬 CLI가 프레젠테이션 레시피와 의존성, 라이트/다크 토큰을 앱으로 복사합니다.",
      ],
      [
        "네이티브 우선",
        "GPUI로 데스크톱 창을 그리고, 같은 컴포넌트가 이 미리보기처럼 웹에서도 돌아갑니다.",
      ],
    ],
  },
} as const;

export function Home({ lang }: { lang: string }) {
  const t = copy[lang === "ko" ? "ko" : "en"];
  return (
    <main className="container mx-auto flex w-full max-w-6xl flex-1 flex-col px-4 py-16 md:py-24">
      <section className="flex flex-col items-center text-center">
        <span className="mb-6 rounded-full border px-3 py-1 text-xs font-medium text-fd-muted-foreground">
          {t.badge}
        </span>
        <h1 className="max-w-3xl text-4xl font-bold tracking-tight md:text-6xl">{t.title}</h1>
        <p className="mt-6 max-w-2xl text-balance text-base text-fd-muted-foreground md:text-lg">
          {t.lead}
        </p>
        <div className="mt-8 flex flex-wrap justify-center gap-3">
          <Link
            href={`/${lang}/docs`}
            className="rounded-md bg-fd-primary px-5 py-2.5 text-sm font-medium text-fd-primary-foreground transition-opacity hover:opacity-90"
          >
            {t.start}
          </Link>
          <Link
            href={`/${lang}/docs/components`}
            className="rounded-md border px-5 py-2.5 text-sm font-medium transition-colors hover:bg-fd-accent"
          >
            {t.browse}
          </Link>
        </div>
      </section>
      <section className="mx-auto mt-16 w-full max-w-4xl">
        <ComponentPreview name="showcase" height={420} />
      </section>
      <section className="mt-16 grid gap-4 md:grid-cols-3">
        {t.features.map(([title, body]) => (
          <div key={title} className="rounded-lg border p-6">
            <h2 className="font-semibold">{title}</h2>
            <p className="mt-2 text-sm text-fd-muted-foreground">{body}</p>
          </div>
        ))}
      </section>
    </main>
  );
}
