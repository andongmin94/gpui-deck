import { Callout } from "fumadocs-ui/components/callout";
import type { ReactNode } from "react";

/** Wraps a Korean page body with a notice that links to the English original. */
export function TranslationNotice({
  original,
  translated,
  children,
}: {
  original: string;
  translated: boolean;
  children?: ReactNode;
}) {
  return (
    <>
      <Callout type={translated ? "info" : "warn"}>
        {translated
          ? "이 페이지는 영어 원문을 번역한 것으로 오류가 있을 수 있습니다. "
          : "이 페이지는 아직 번역되지 않아 영어 원문을 보여 줍니다. "}
        <a href={original}>영어 원문 보기</a>
      </Callout>
      {children}
    </>
  );
}
