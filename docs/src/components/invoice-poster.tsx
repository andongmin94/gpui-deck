import "./invoice-poster.css";

const descriptions = {
  en: "Billing overview with $11,750 collected and $4,650 outstanding. Monthly collections rise from January through June. The table lists eight invoices and their payment status.",
  ko: "청구 현황 화면에 수금액 11,750달러와 미수금 4,650달러가 표시됩니다. 차트에서 1월부터 6월까지 수금액이 증가하는 추이를, 표에서 청구서 8건의 결제 상태를 볼 수 있습니다.",
  ja: "請求管理画面。入金済みは11,750ドル、未収金は4,650ドルです。グラフは1月から6月にかけての入金額の増加を示し、表には8件の請求書と支払い状況が並びます。",
  zh: "账单概览，已收款 11,750 美元，待收款 4,650 美元。图表显示收款额从一月到六月逐月增长，表格列出八张账单及其付款状态。",
};

export function InvoicePoster({ lang }: { lang: string }) {
  return (
    <div
      className="invoice-poster"
      // oxlint-disable-next-line jsx-a11y/prefer-tag-over-role -- CSS loads only the capture for the active site theme and viewport.
      role="img"
      aria-label={descriptions[lang as keyof typeof descriptions] ?? descriptions.en}
    />
  );
}
