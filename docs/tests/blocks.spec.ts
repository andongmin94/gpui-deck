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
