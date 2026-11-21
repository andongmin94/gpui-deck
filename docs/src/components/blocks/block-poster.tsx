import type { Block } from "./block-copy";

export function BlockPoster({ block, alt }: { block: Block; alt: string }) {
  return (
    <picture>
      <source
        media="(max-width: 640px)"
        srcSet={`${block.poster}-mobile.webp`}
        width={390}
        height={block.mobileHeight}
      />
      <img
        src={`${block.poster}.webp`}
        alt={alt}
        width={1440}
        height={820}
        className="block h-auto w-full"
      />
    </picture>
  );
}
