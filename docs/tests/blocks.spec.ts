import { readFile } from "node:fs/promises";
import { expect, test } from "@playwright/test";

for (const [slug, title] of [
  ["login", "Login"],
  ["sidebar-settings", "Sidebar settings"],
  ["billing-dashboard", "Billing dashboard"],
]) {
  test(`browse, inspect and copy the installable ${slug} block`, async ({ page, context }) => {
    const command = `gpui-deck add https://github.com/andongmin94/gpui-deck/blob/main/blocks/${slug}/block.json`;
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    // Exercise the launch control independently of the host GPU. The frame uses its own backend.
    await page.addInitScript(() => {
      if (window === window.top) {
        Object.defineProperty(navigator, "gpu", { value: { requestAdapter: async () => ({}) } });
      }
    });
    const runtimeRequests: string[] = [];
    page.on("request", (request) => {
      const url = new URL(request.url());
      if (url.pathname.startsWith("/preview")) runtimeRequests.push(url.pathname + url.search);
    });
    await page.goto("/en/blocks");
    await page.getByRole("link", { name: title, exact: true }).click();
    await expect(page).toHaveURL(new RegExp(`/en/blocks/${slug}/?$`));
    await expect(page.getByRole("heading", { level: 1 })).toHaveText(title);
    await expect(page.locator(`img[src="/images/blocks/${slug}.webp"]`)).toBeVisible();
    await expect(page.locator("iframe")).toHaveCount(0);
    if (slug === "billing-dashboard") {
      const starter = page.getByRole("region", { name: "Run as a new app" });
      const download = starter.getByRole("link", { name: "Download project (.zip)" });
      await expect(download).toHaveAttribute("href", "/downloads/billing-dashboard.zip");
      await expect(starter.getByText("cargo run", { exact: true })).toBeVisible();
      const archive = await page.request.get((await download.getAttribute("href"))!);
      expect(archive.ok()).toBe(true);
      expect((await archive.body()).subarray(0, 4)).toEqual(Buffer.from([0x50, 0x4b, 3, 4]));
      const existing = page.getByRole("region", { name: "Add to an existing app" });
      await expect(existing.getByRole("textbox", { name: "Install command" })).toHaveValue(command);
      await expect(
        existing.getByRole("link", { name: "Connect the dashboard to your window" }),
      ).toHaveAttribute("href", "/en/docs/guides/blocks#existing-app");
    }

    const picker = page.getByRole("combobox", { name: "Source file" });
    await expect(async () => {
      await page.getByRole("tab", { name: "Rust source" }).click();
      await expect(picker).toBeVisible({ timeout: 1000 });
    }).toPass();
    const directory = new URL(`../../blocks/${slug}/`, import.meta.url);
    const manifest: { files: { target: string; source: string }[] } = JSON.parse(
      await readFile(new URL("block.json", directory), "utf8"),
    );
    for (const { target, source } of manifest.files.filter((file) => file.target.endsWith(".rs"))) {
      await picker.selectOption({ label: target });
      const original = await readFile(new URL(source, directory), "utf8");
      await expect(page.getByRole("tabpanel").locator("pre")).toHaveText(original);
    }

    await page.getByRole("tab", { name: "Preview", exact: true }).click();
    await page.getByRole("button", { name: "Copy install command" }).click();
    await expect
      .poll(() =>
        page.evaluate(
          async (expected) => (await navigator.clipboard.readText()) === expected,
          command,
        ),
      )
      .toBe(true);
    expect(runtimeRequests).toEqual([]);
    await page.getByRole("button", { name: "Run preview" }).click();
    await expect(page.locator("iframe")).toHaveAttribute("src", `/preview?c=${slug}`);
    await expect.poll(() => runtimeRequests).toContain(`/preview?c=${slug}`);
  });
}

test("the settings card links to the Switch documentation", async ({ page }) => {
  await page.goto("/en/blocks");
  await page.getByRole("link", { name: "Switch", exact: true }).click();
  await expect(page.getByRole("heading", { name: "Checkbox and Switch", level: 1 })).toBeVisible();
});

test("purpose and style filters reveal installable registry screens and survive navigation", async ({
  page,
}) => {
  const pageErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/ko/blocks?purpose=dashboards&style=neumorphism");
  const purpose = page.getByRole("combobox", { name: "용도" });
  const style = page.getByRole("combobox", { name: "스타일" });
  await expect(style).toHaveValue("neumorphism");
  await expect(purpose).toHaveValue("dashboards");
  const articles = page.getByRole("article");
  await expect(articles).toHaveCount(1);
  await expect(
    articles.getByRole("link", { name: "분석 대시보드 · Neumorphism UI" }),
  ).toHaveAttribute("href", "/ko/directory/neumorphism#template-analytics");
  await expect(articles.getByRole("textbox", { name: "설치 명령" })).toHaveValue(
    /\/registries\/neumorphism\/template-analytics\.json$/,
  );
  await articles.getByRole("tab", { name: "Rust 소스" }).click();
  await expect(articles.getByRole("tabpanel").locator("pre")).toHaveText(
    await readFile(
      new URL("../../registries/neumorphism/items/template-analytics.rs", import.meta.url),
      "utf8",
    ),
  );
  await style.selectOption("");
  await expect(articles).toHaveCount(3);
  await purpose.selectOption("authentication");
  await expect(articles).toHaveCount(2);
  await style.selectOption("neobrutal");
  await expect(articles).toHaveCount(1);
  await expect(articles.getByRole("textbox", { name: "설치 명령" })).toHaveValue(
    /\/registries\/neobrutal\/login\.json$/,
  );
  await style.selectOption("neumorphism");
  await expect(articles).toHaveCount(0);
  await expect(page.getByRole("button", { name: "필터 초기화" })).toBeVisible();
  await page.goBack();
  await expect(style).toHaveValue("neobrutal");
  await expect(page.locator("iframe")).toHaveCount(0);
  expect(await page.locator("html").evaluate((el) => el.scrollWidth <= innerWidth)).toBe(true);
  expect(pageErrors).toEqual([]);
});

test("the gallery previews, installs and runs billing inline on a phone", async ({
  page,
  context,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.addInitScript(() => {
    if (window === window.top) {
      Object.defineProperty(navigator, "gpu", { value: { requestAdapter: async () => ({}) } });
    }
  });
  const runtimes: string[] = [];
  page.on("request", (request) => {
    const path = new URL(request.url()).pathname;
    if (path.startsWith("/preview")) runtimes.push(path);
  });
  await page.goto("/en/blocks");
  const billing = page.getByRole("article", { name: "Billing dashboard" });
  await expect(page.getByRole("article").first()).toHaveAttribute("id", "billing-dashboard");
  await expect
    .poll(() => billing.getByRole("img").evaluate((el: HTMLImageElement) => el.currentSrc))
    .toContain("billing-dashboard-mobile.webp");
  await billing.getByRole("tab", { name: "Rust source" }).click();
  const picker = billing.getByRole("combobox", { name: "Source file" });
  await expect(picker).toBeVisible();
  await expect(billing.getByRole("tabpanel").locator("pre")).toContainText("BillingDashboard");
  await billing.getByRole("button", { name: "Copy install command" }).click();
  await expect
    .poll(() => page.evaluate(() => navigator.clipboard.readText()))
    .toBe(
      "gpui-deck add https://github.com/andongmin94/gpui-deck/blob/main/blocks/billing-dashboard/block.json",
    );
  await billing.getByRole("tab", { name: "Preview", exact: true }).click();
  expect(runtimes).toEqual([]);
  await billing.getByRole("button", { name: "Run preview" }).click();
  await expect(billing.locator("iframe")).toHaveAttribute("src", "/preview?c=billing-dashboard");
  await expect(page.locator("iframe")).toHaveCount(1);
  await expect(page).toHaveURL(/\/en\/blocks\/?$/);
});

test("the block image and Rust source stay available when no adapter is found", async ({
  page,
}) => {
  await page.addInitScript(() => {
    if (window === window.top) {
      Object.defineProperty(navigator, "gpu", { value: { requestAdapter: async () => null } });
    }
  });
  await page.goto("/ko/blocks/billing-dashboard");
  const preview = page.getByRole("tabpanel", { name: "미리보기" });
  await expect(async () => {
    await page.getByRole("button", { name: "미리보기 실행" }).click();
    await expect(preview.getByRole("status")).toContainText("WebGPU", { timeout: 1000 });
  }).toPass();
  await expect(preview.getByRole("img")).toBeVisible();
  await expect(page.locator("iframe")).toHaveCount(0);
  await page.getByRole("tab", { name: "Rust 소스" }).click();
  await expect(page.getByRole("combobox", { name: "소스 파일" })).toBeVisible();
});
