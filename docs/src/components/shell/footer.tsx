import { GIT } from "@/lib/site";
import { copyFor } from "./copy";

export function SiteFooter({ lang }: { lang: string }) {
  return (
    <footer className="py-6">
      <p className="mx-auto max-w-[1400px] px-4 text-center text-xs text-fd-muted-foreground md:px-6">
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
    </footer>
  );
}
