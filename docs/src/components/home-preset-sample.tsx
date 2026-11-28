import { Activity, Bookmark } from "lucide-react";
import { useState } from "react";
import type { Preset } from "@/lib/create-preset";
import "./home-preset-sample.css";

const copy = {
  en: {
    activity: "Release activity",
    published: "published releases",
    reports: "Saved reports",
    name: "Report name",
    save: "Save report",
    october: "October activity",
    november: "November activity",
  },
  ko: {
    activity: "릴리스 현황",
    published: "건 배포",
    reports: "저장한 보고서",
    name: "보고서 이름",
    save: "보고서 저장",
    october: "10월 릴리스 현황",
    november: "11월 릴리스 현황",
  },
  ja: {
    activity: "リリース状況",
    published: "件のリリース",
    reports: "保存したレポート",
    name: "レポート名",
    save: "レポートを保存",
    october: "10月のリリース状況",
    november: "11月のリリース状況",
  },
  zh: {
    activity: "发布动态",
    published: "次发布",
    reports: "已保存的报告",
    name: "报告名称",
    save: "保存报告",
    october: "10月发布动态",
    november: "11月发布动态",
  },
};

// Published series from the Create gallery, June through November.
const releases = [6, 9, 7, 13, 10, 15];
const total = releases.reduce((sum, count) => sum + count, 0);
const highest = Math.max(...releases);

export function HomePresetSample({ lang, preset }: { lang: string; preset: Preset }) {
  const t = copy[lang as keyof typeof copy] ?? copy.en;
  const months = new Intl.DateTimeFormat(lang, { month: "short", timeZone: "UTC" });
  const [name, setName] = useState(t.november);
  const [saved, setSaved] = useState(t.october);
  return (
    <div
      className="home-preset-sample @container min-w-0 rounded-3xl p-4 sm:p-5"
      data-style={preset.style}
      data-radius={preset.radius}
      data-swatch={preset.accent}
    >
      <div className="grid min-w-0 items-start gap-4 @min-[28rem]:grid-cols-2">
        <section className="sample-panel min-w-0 p-4 sm:p-5" aria-labelledby="sample-activity">
          <div className="flex items-center justify-between gap-2">
            <h2 id="sample-activity" className="text-xl leading-6 font-semibold tracking-tight">
              {t.activity}
            </h2>
            <Activity className="sample-icon size-5 shrink-0" aria-hidden />
          </div>
          <p className="mt-4 flex flex-wrap items-baseline gap-x-2">
            <strong className="text-4xl leading-none font-semibold tracking-tight tabular-nums">
              {total}
            </strong>
            <span className="sample-muted text-sm">{t.published}</span>
          </p>
          <ol className="mt-5 grid grid-cols-6 gap-1 text-center text-xs tabular-nums">
            {releases.map((count, index) => {
              const month = months.format(Date.UTC(2024, index + 5));
              return (
                <li
                  key={month}
                  title={`${month} · ${count}`}
                  className="flex min-w-0 flex-col items-center"
                >
                  <div className="flex h-20 w-full max-w-8 flex-col justify-end">
                    <div
                      className="sample-bar"
                      style={{ height: `${(count / highest) * 100}%` }}
                      aria-hidden
                    />
                  </div>
                  <span className="sample-muted mt-1.5">
                    {month}
                    <span className="sr-only">: {count}</span>
                  </span>
                </li>
              );
            })}
          </ol>
        </section>
        <section className="sample-panel min-w-0 p-4 sm:p-5" aria-labelledby="sample-reports">
          <div className="flex items-center justify-between gap-2">
            <h2 id="sample-reports" className="text-xl leading-6 font-semibold tracking-tight">
              {t.reports}
            </h2>
            <Bookmark className="sample-icon size-5 shrink-0" aria-hidden />
          </div>
          <form
            className="mt-4 grid gap-3"
            onSubmit={(event) => {
              event.preventDefault();
              const trimmed = name.trim();
              if (trimmed) {
                setSaved(trimmed);
                setName("");
              }
            }}
          >
            <label className="grid gap-1.5 text-sm font-medium">
              {t.name}
              <input
                className="sample-field h-10 min-w-0 w-full px-3 text-sm font-normal"
                value={name}
                placeholder={t.november}
                onChange={(event) => setName(event.target.value)}
              />
            </label>
            <button
              className="sample-action min-h-10 px-3 text-sm font-semibold hover:opacity-90 disabled:opacity-50"
              type="submit"
              disabled={!name.trim()}
            >
              {t.save}
            </button>
            <output className="sample-saved flex min-w-0 items-center gap-2 border-t pt-3 text-sm">
              <Bookmark className="size-4 shrink-0" aria-hidden />
              <span className="truncate">{saved}</span>
            </output>
          </form>
        </section>
      </div>
    </div>
  );
}
