import { expect, test } from "@playwright/test";

test("the root opens the English home page", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveURL(/\/en\/?$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Build desktop apps with Rust and GPUI.",
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
    "https://github.com/andongmin94/gpui-design/blob/main/crates/web-preview/src/demos/invoice_workspace.rs",
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
  const picker = page.getByRole("button", { name: /^강조색/ });
  await expect(picker).toHaveAccessibleName("강조색 · 뉴트럴");
  await expect(async () => {
    await picker.press("ArrowDown");
    await expect(page.getByRole("menuitemradio", { name: "뉴트럴" })).toBeChecked({
      timeout: 1000,
    });
  }).toPass();
  await page.getByRole("menuitemradio", { name: "바이올렛" }).click();
  const poster = page.getByRole("img", { name: /청구 현황/ });
  await expect(poster).toHaveCSS("background-image", /\/violet\/mobile-light\.webp/);

  const resources: string[] = [];
  page.on("request", (request) => resources.push(new URL(request.url()).pathname));
  await page.reload();
  await expect(picker).toHaveAccessibleName("강조색 · 바이올렛");
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

test("a component page embeds its live preview", async ({ page }) => {
  await page.goto("/en/docs/components/button");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Button");
  await expect(page.locator('iframe[title="button preview"]')).toHaveAttribute(
    "src",
    /\/preview\?c=button$/,
  );
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
