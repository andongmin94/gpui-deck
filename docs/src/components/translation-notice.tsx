import { Callout } from "fumadocs-ui/components/callout";
import type { ReactNode } from "react";

const notices = {
  ko: {
    translated: "이 페이지는 영어 원문을 번역한 것으로 오류가 있을 수 있습니다.",
    fallback: "이 페이지는 아직 번역되지 않아 영어 원문을 보여 줍니다.",
    link: "영어 원문 보기",
  },
  ja: {
    translated: "このページは英語の原文を翻訳したもので、誤りを含む場合があります。",
    fallback: "このページはまだ翻訳されていないため、英語の原文を表示しています。",
    link: "英語の原文を見る",
  },
  zh: {
    translated: "本页译自英文原文，可能存在错误。",
    fallback: "本页尚未翻译，显示的是英文原文。",
    link: "查看英文原文",
  },
} as const;

/** Wraps a translated page body with a notice that links to the English original. */
export function TranslationNotice({
  lang,
  original,
  translated,
  children,
}: {
  lang: string;
  original: string;
  translated: boolean;
  children?: ReactNode;
}) {
  const t = notices[lang as keyof typeof notices] ?? notices.ko;
  return (
    <>
      <Callout type={translated ? "info" : "warn"}>
        {translated ? t.translated : t.fallback} <a href={original}>{t.link}</a>
      </Callout>
      {children}
    </>
  );
}
