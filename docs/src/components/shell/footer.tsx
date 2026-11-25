import { GIT } from "@/lib/site";
import { Link } from "fumapress/client";
import { copyFor } from "./copy";

export function SiteFooter({ lang }: { lang: string }) {
  return (
    <footer className="mx-auto flex w-full max-w-[100rem] flex-wrap items-center justify-between gap-2 px-4 py-6 text-xs text-fd-muted-foreground sm:px-6">
      <p>
        {copyFor(lang).footer}{" "}
        <a
          href={`https://github.com/${GIT.user}/${GIT.repo}`}
          target="_blank"
          rel="noreferrer"
          className="font-medium underline underline-offset-4"
        >
          GitHub
        </a>
        .
      </p>
      <Link href={`/${lang}/docs/guides/blocks#publish`} className="font-medium hover:underline">
        {copyFor(lang).publish}
      </Link>
    </footer>
  );
}
