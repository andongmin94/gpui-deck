import { billing } from "./block-copy";

export function BlockPoster({ alt }: { alt: string }) {
  return (
    <picture>
      <source
        media="(max-width: 640px)"
        srcSet={`${billing.poster}-mobile.webp`}
        width={390}
        height={1360}
      />
      <img
        src={`${billing.poster}.webp`}
        alt={alt}
        width={1440}
        height={820}
        className="block h-auto w-full"
      />
    </picture>
  );
}
