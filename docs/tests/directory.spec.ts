import { readFile } from "node:fs/promises";
import { expect, test } from "@playwright/test";

test("every registry item ships a desktop and mobile preview", async ({ request }) => {
  const url = new URL("../../registries/neobrutal/registry.json", import.meta.url);
  const catalog = JSON.parse(await readFile(url, "utf8")) as { items: { manifest: string }[] };
  for (const { manifest } of catalog.items) {
    const slug = manifest.replace(/\.json$/, "");
    const urls = ["", "-mobile"].map(
      (suffix) => `/images/registries/neobrutal/${slug}${suffix}.webp`,
    );
    for (const response of await Promise.all(urls.map((url) => request.head(url)))) {
      expect(response.status(), response.url()).toBe(200);
      expect(response.headers()["content-type"], response.url()).toContain("image/webp");
    }
  }
});

test("discover the Neobrutal registry, inspect its Rust and copy standalone installers", async ({
  page,
  context,
}) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  const runtime: string[] = [];
  page.on("request", (request) => {
    if (new URL(request.url()).pathname.startsWith("/preview")) runtime.push(request.url());
  });
  await page.goto("/en");
  await page
    .getByRole("navigation", { name: "Menu", exact: true })
    .getByRole("link", { name: "Directory" })
    .click();
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Registry directory");
  await page.getByRole("link", { name: "Input", exact: true }).click();
  await expect(page).toHaveURL(/\/en\/directory\/neobrutal\/?#input$/);
  await expect(page.getByRole("heading", { name: "Input", exact: true })).toBeInViewport();

  const directory = new URL("../../registries/neobrutal/", import.meta.url);
  for (const [slug, title] of [
    ["login", "Login"],
    ["button", "Button"],
    ["input", "Input"],
    ["chart-build-duration", "Build Duration"],
    ["theme-blue", "Blue"],
  ]) {
    await page
      .getByRole("navigation", { name: "Browse the registry" })
      .getByRole("link", { name: title, exact: true })
      .click();
    const item = page.getByRole("region", { name: title, exact: true });
    const command = `gpui-deck add https://github.com/andongmin94/gpui-deck/blob/main/registries/neobrutal/${slug}.json`;
    await expect(item.getByRole("textbox", { name: "Install command" })).toHaveValue(command);
    await item.getByRole("button", { name: "Copy install command" }).click();
    await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(command);
    await item.getByRole("tab", { name: "Rust source" }).click();
    const manifest = JSON.parse(await readFile(new URL(`${slug}.json`, directory), "utf8")) as {
      files: { source: string; target: string }[];
    };
    const source =
      manifest.files.find((file) => file.source.endsWith(`${slug}.rs`)) ??
      manifest.files.find((file) => file.target === "mod.rs")!;
    await item.getByRole("combobox", { name: "Source file" }).selectOption(source.source);
    await expect(item.locator("pre")).toHaveText(
      await readFile(new URL(source.source, directory), "utf8"),
    );
  }
  expect(runtime).toEqual([]);
});

test("search the Korean registry on mobile and return to the previous item", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/ko/directory/neobrutal#login");
  const search = page.getByRole("searchbox");
  await search.fill("테마");
  const select = page.getByRole("combobox", { name: "항목 선택" });
  await expect(select.locator("optgroup option")).toHaveCount(19);
  await select.selectOption("theme-blue");
  await expect(page.getByRole("region", { name: "블루", exact: true })).toBeVisible();
  await page.goBack();
  await expect(page.getByRole("heading", { name: "로그인", exact: true })).toBeVisible();
  await search.fill("no-such-neobrutal-item");
  await expect(page.getByText("검색 결과가 없습니다.")).toBeVisible();
});
