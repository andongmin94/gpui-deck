import { Link } from "fumapress/client";

import { ArrowRight } from "lucide-react";

import { ComponentPreview } from "./preview";
import { SiteFooter } from "./shell/footer";
import { SiteHeader } from "./shell/header";

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
    <div className="flex min-h-svh flex-col">
      <SiteHeader lang={lang} />
      <main id="main-content" className="flex-1">
        <section className="mx-auto flex max-w-3xl flex-col items-center gap-2 px-4 pt-16 pb-10 text-center md:pt-20">
          <Link
            href={`/${lang}/docs/components`}
            className="mb-2 inline-flex items-center gap-1 rounded-full bg-fd-secondary px-3 py-1 text-xs font-medium transition-colors hover:bg-fd-accent"
          >
            {t.badge}
            <ArrowRight className="size-3" />
          </Link>
          <h1 className="text-4xl leading-tight font-semibold tracking-tight text-balance md:text-5xl lg:leading-[1.1]">
            {t.title}
          </h1>
          <p className="max-w-2xl text-base text-balance text-fd-foreground/80 sm:text-lg">
            {t.lead}
          </p>
          <div className="mt-4 flex flex-wrap justify-center gap-2">
            <Link
              href={`/${lang}/docs`}
              className="inline-flex h-9 items-center rounded-full bg-fd-primary px-4 text-sm font-medium text-fd-primary-foreground transition-opacity hover:opacity-90"
            >
              {t.start}
            </Link>
            <Link
              href={`/${lang}/docs/components`}
              className="inline-flex h-9 items-center rounded-full px-4 text-sm font-medium transition-colors hover:bg-fd-accent"
            >
              {t.browse}
            </Link>
          </div>
        </section>
        <section className="mx-auto w-full max-w-5xl px-4">
          <ComponentPreview name="showcase" height={460} />
        </section>
        <section className="mx-auto grid w-full max-w-5xl gap-4 px-4 pt-6 pb-16 md:grid-cols-3">
          {t.features.map(([title, body]) => (
            <div key={title} className="rounded-xl border p-6">
              <h2 className="font-semibold">{title}</h2>
              <p className="mt-2 text-sm text-fd-muted-foreground">{body}</p>
            </div>
          ))}
        </section>
      </main>
      <SiteFooter lang={lang} />
    </div>
  );
}
