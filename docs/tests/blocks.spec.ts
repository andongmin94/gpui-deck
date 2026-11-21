import { readFile } from "node:fs/promises";
import { expect, test } from "@playwright/test";

test("browse, inspect and copy the installable billing dashboard", async ({ page, context }) => {
  const command =
    "gpui-deck add https://github.com/andongmin94/gpui-deck/blob/main/blocks/billing-dashboard/block.json";
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
  await page.getByRole("link", { name: "Billing dashboard", exact: true }).click();
  await expect(page).toHaveURL(/\/en\/blocks\/billing-dashboard\/?$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Billing dashboard");
  await expect(page.locator('img[src="/images/blocks/billing-dashboard.webp"]')).toBeVisible();
  await expect(page.locator("iframe")).toHaveCount(0);

  const picker = page.getByRole("combobox", { name: "Source file" });
  await expect(async () => {
    await page.getByRole("tab", { name: "Rust source" }).click();
    await expect(picker).toBeVisible({ timeout: 1000 });
  }).toPass();
  for (const [installed, source] of [
    ["mod.rs", "dashboard.rs"],
    ["palette.rs", "palette.rs"],
    ["components/panel.rs", "components/panel.rs"],
  ]) {
    await picker.selectOption({ label: installed });
    const original = await readFile(
      new URL(`../../blocks/billing-dashboard/${source}`, import.meta.url),
      "utf8",
    );
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
  await expect(page.locator("iframe")).toHaveAttribute("src", "/preview?c=billing-dashboard");
  await expect.poll(() => runtimeRequests).toContain("/preview?c=billing-dashboard");
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
