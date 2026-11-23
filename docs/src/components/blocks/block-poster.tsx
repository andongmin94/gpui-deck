import type { PreviewItem } from "./block-copy";

export function BlockPoster({ block, alt }: { block: PreviewItem; alt: string }) {
  return (
    <picture>
      {block.mobileHeight && (
        <source
          media="(max-width: 640px)"
          srcSet={`${block.poster}-mobile.webp`}
          width={390}
          height={block.mobileHeight}
        />
      )}
      <img
        src={`${block.poster}.webp`}
        alt={alt}
        width={block.posterWidth}
        height={block.posterHeight}
        loading="lazy"
        className="block h-auto w-full"
      />
    </picture>
  );
}
