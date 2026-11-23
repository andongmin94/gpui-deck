import "./invoice-poster.css";

const descriptions = {
  en: "Monthly collections rise from January to $3,200 in June. The invoice table shows each customer, payment status and amount.",
  ko: "청구 현황. 월별 수금액이 1월부터 늘어 6월에는 3,200달러에 이릅니다. 청구서 표에는 고객명, 결제 상태, 금액이 표시됩니다.",
  ja: "月ごとの入金額は1月から増加し、6月には3,200ドルに達しています。請求書の表には顧客名、支払い状況、金額が並びます。",
  zh: "月度收款额从一月起逐月增加，六月达到 3,200 美元。账单表格列出了客户、付款状态和金额。",
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
