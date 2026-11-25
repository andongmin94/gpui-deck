import { expect, test } from "@playwright/test";

test("the root opens the English home page", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveURL(/\/en\/?$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Build your design system with GPUI.",
  );
  const main = page.getByRole("main");
  for (const [title, slug] of [
    ["Billing dashboard", "billing-dashboard"],
    ["Login", "login"],
    ["Sidebar settings", "sidebar-settings"],
  ]) {
    await expect(main.getByRole("link", { name: title, exact: true })).toHaveAttribute(
      "href",
      `/en/blocks/${slug}`,
    );
  }
  await main.getByRole("combobox", { name: "Style", exact: true }).selectOption("neumorphism");
  await main.getByRole("combobox", { name: "Radius", exact: true }).selectOption("large");
  await main.getByRole("radio", { name: "Neutral", exact: true }).focus();
  await page.keyboard.press("ArrowLeft");
  await expect(main.getByRole("radio", { name: "Sage", exact: true })).toBeChecked();
  await main.getByRole("link", { name: "Open preset", exact: true }).click();
  await expect(page).toHaveURL(
    /\/en\/create\?preset=v1\.neumorphism\.neutral\.sage\.neutral\.pretendard\.pretendard\.outline\.large$/,
  );
  await expect(page.getByRole("combobox", { name: "Style", exact: true })).toHaveValue(
    "neumorphism",
  );
});

test("the invoice workspace loads GPUI only when launched and links to its implementation", async ({
  page,
}) => {
  const previewRequests: string[] = [];
  page.on("request", (request) => {
    if (new URL(request.url()).pathname.startsWith("/preview")) previewRequests.push(request.url());
  });
  // This test covers when the app starts. The response stands in for the GPU backend.
  await page.addInitScript(() => {
    if (!("gpu" in navigator)) Object.defineProperty(navigator, "gpu", { value: {} });
  });
  await page.route("**/preview?c=invoice-workspace", (route) =>
    route.fulfill({ contentType: "text/html", body: "<title>Invoice workspace</title>" }),
  );
  await page.goto("/ko");
  const workspace = page.getByRole("region", { name: "청구서 관리" });
  await expect(workspace.getByRole("img")).toBeVisible();
  await expect(workspace.getByRole("link", { name: "Rust 소스 보기" })).toHaveAttribute(
    "href",
    "https://github.com/andongmin94/gpui-deck/blob/main/crates/web-preview/src/demos/invoice_workspace.rs",
  );
  expect(previewRequests).toEqual([]);
  await expect(async () => {
    await workspace.getByRole("button", { name: "직접 실행" }).click();
    await expect(workspace.locator("iframe")).toBeVisible({ timeout: 1000 });
  }).toPass();
  await expect.poll(() => previewRequests.length).toBe(1);
  await workspace.getByRole("button", { name: "미리보기로 돌아가기" }).click();
  await expect(workspace.locator("iframe")).toHaveCount(0);
  await expect(workspace.getByRole("img")).toBeVisible();
});

test("the invoice preview and source stay available without WebGPU", async ({ page }) => {
  await page.addInitScript(() => Reflect.deleteProperty(Navigator.prototype, "gpu"));
  await page.goto("/ko");
  const workspace = page.getByRole("region", { name: "청구서 관리" });
  await expect(async () => {
    await workspace.getByRole("button", { name: "직접 실행" }).click();
    await expect(workspace.getByRole("status")).toContainText("WebGPU", { timeout: 1000 });
  }).toPass();
  await expect(workspace.getByRole("img")).toBeVisible();
  await expect(workspace.getByRole("link", { name: "Rust 소스 보기" })).toBeVisible();
  await expect(workspace.locator("iframe")).toHaveCount(0);
});

test("the invoice accent follows the picker and survives a reload before the demo starts", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/ko");
  const poster = page.getByRole("img", { name: /청구 현황/ });
  await expect(poster).toBeInViewport({ ratio: 0.75 });
  const trigger = page.getByRole("button", { name: "탐색 열기/닫기" });
  const menu = page.getByRole("dialog", { name: "메뉴" });
  await expect(async () => {
    await trigger.click();
    await expect(menu).toBeVisible({ timeout: 1000 });
  }).toPass();
  const picker = menu.getByRole("button", { name: /^강조색/ });
  await expect(picker).toHaveAccessibleName("강조색 · 뉴트럴");
  await picker.press("ArrowDown");
  await expect(page.getByRole("menuitemradio", { name: "뉴트럴" })).toBeChecked();
  await page.getByRole("menuitemradio", { name: "바이올렛" }).click();
  await expect(picker).toHaveAccessibleName("강조색 · 바이올렛");
  await page.keyboard.press("Escape");
  await expect(menu).toBeHidden();
  await expect(poster).toHaveCSS("background-image", /\/violet\/mobile-light\.webp/);

  const resources: string[] = [];
  page.on("request", (request) => resources.push(new URL(request.url()).pathname));
  await page.reload();
  await expect(poster).toHaveCSS("background-image", /\/violet\/mobile-light\.webp/);
  expect(resources.filter((path) => path.startsWith("/images/invoices/"))).toEqual([
    "/images/invoices/violet/mobile-light.webp",
  ]);
  expect(resources.some((path) => path.startsWith("/preview"))).toBe(false);
});

test("the Korean home page and catalogue cards lead to the Korean component", async ({ page }) => {
  await page.goto("/ko");
  await expect(page.locator("html")).toHaveAttribute("lang", "ko");
  await page.getByRole("link", { name: "시작하기" }).click();
  await expect(page).toHaveURL(/\/ko\/docs\/?$/);
  const article = page.getByRole("article");
  await article.getByRole("link", { name: /^컴포넌트/ }).click();
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("전체 컴포넌트");
  await article.getByRole("link", { name: /^Chart/ }).click();
  await expect(page).toHaveURL(/\/ko\/docs\/components\/chart\/?$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Chart");
});

test("component and recipe previews follow the selected accent without restarting", async ({
  page,
}) => {
  await page.goto("/en/docs/components/button");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Button");
  const button = page.locator('iframe[title="button preview"]');
  await expect(button).toHaveAttribute("data-accent", "neutral");
  await expect(button).toHaveAttribute("src", "/preview?c=button");
  await button.scrollIntoViewIfNeeded();
  await expect(button.contentFrame().locator("title")).toHaveJSProperty(
    "text",
    "GPUI Deck preview",
  );
  const original = await button.evaluateHandle((frame: HTMLIFrameElement) => frame.contentDocument);
  const picker = page.getByRole("button", { name: /^Accent color/ });
  await expect(async () => {
    await picker.click();
    await expect(page.getByRole("menuitemradio", { name: "Rose" })).toBeVisible({ timeout: 1000 });
  }).toPass();
  await page.getByRole("menuitemradio", { name: "Rose" }).click();
  await expect(button).toHaveAttribute("data-accent", "rose");
  await expect(button).toHaveAttribute("src", "/preview?c=button");
  expect(
    await button.evaluate(
      (frame: HTMLIFrameElement, doc) => frame.contentDocument === doc,
      original,
    ),
  ).toBe(true);

  await page.goto("/en/docs/recipes/badge");
  const badge = page.locator('iframe[title="badge preview"]');
  await expect(badge).toHaveAttribute("data-accent", "rose");
  await picker.click();
  await page.getByRole("menuitemradio", { name: "Neutral" }).click();
  await expect(badge).toHaveAttribute("data-accent", "neutral");
  await expect(badge).toHaveAttribute("src", "/preview?c=badge");
});

for (const lang of ["ko", "ja", "zh"]) {
  test(`the ${lang} button page is served in its language`, async ({ page }) => {
    await page.goto(`/${lang}/docs/components/button`);
    await expect(page.locator("html")).toHaveAttribute("lang", lang);
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Button");
  });
}

test("search finds a component page", async ({ page }) => {
  await page.goto("/en/docs");
  const input = page.getByRole("dialog").getByRole("combobox");
  // The trigger only opens the dialog once the page has hydrated.
  await expect(async () => {
    await page
      .getByRole("button", { name: /search/i })
      .first()
      .click();
    await expect(input).toBeVisible({ timeout: 1000 });
  }).toPass();
  await input.fill("carousel");
  await expect(page.getByRole("dialog").getByText("Carousel").first()).toBeVisible();
});
