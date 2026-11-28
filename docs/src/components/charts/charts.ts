import type { Registry } from "../directory/registries";

export const chartTypes = [
  "bar",
  "horizontal-bar",
  "stacked-bar",
  "line",
  "area",
  "stacked-area",
  "donut",
] as const;
export type ChartExample = {
  item: Registry["items"][number];
  type: (typeof chartTypes)[number];
  title: string;
  registry: string;
  href: string;
};

const labels = {
  en: {
    types: {
      all: "All",
      bar: "Bar",
      "horizontal-bar": "Horizontal bar",
      "stacked-bar": "Stacked bar",
      line: "Line",
      area: "Area",
      "stacked-area": "Stacked area",
      donut: "Donut",
    },
    examples: { neobrutal: "Print orders", neumorphism: "Visits over time" },
    filter: "Chart type",
    docs: "Chart API",
  },
  ko: {
    types: {
      all: "전체",
      bar: "막대",
      "horizontal-bar": "가로 막대",
      "stacked-bar": "누적 막대",
      line: "선",
      area: "영역",
      "stacked-area": "누적 영역",
      donut: "도넛",
    },
    examples: { neobrutal: "인쇄 주문", neumorphism: "일별 방문 수" },
    filter: "차트 유형",
    docs: "차트 API",
  },
  ja: {
    types: {
      all: "すべて",
      bar: "棒",
      "horizontal-bar": "横棒",
      "stacked-bar": "積み上げ棒",
      line: "折れ線",
      area: "面",
      "stacked-area": "積み上げ面",
      donut: "ドーナツ",
    },
    examples: { neobrutal: "印刷注文", neumorphism: "日別の訪問数" },
    filter: "チャートの種類",
    docs: "チャート API",
  },
  zh: {
    types: {
      all: "全部",
      bar: "柱状图",
      "horizontal-bar": "条形图",
      "stacked-bar": "堆叠柱状图",
      line: "折线图",
      area: "面积图",
      "stacked-area": "堆叠面积图",
      donut: "环形图",
    },
    examples: { neobrutal: "印刷订单", neumorphism: "每日访问量" },
    filter: "图表类型",
    docs: "图表 API",
  },
};

export function chartCopy(lang: string) {
  return labels[lang as keyof typeof labels] ?? labels.en;
}
