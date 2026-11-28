import { readFile } from "node:fs/promises";
import { expect, test } from "@playwright/test";

test("a chart type links to the matching renderer, install command and Rust source", async ({
  page,
  context,
}) => {
  const pageErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/ko/charts?type=stacked-area");
  const filters = page.getByRole("navigation", { name: "차트 유형" });
  const example = page.getByRole("article");
  await expect(filters.getByRole("link", { name: "누적 영역", exact: true })).toHaveAttribute(
    "aria-current",
    "page",
  );
  await expect(example).toHaveCount(1);
  await expect(example).toContainText("Neumorphism UI");
  await expect(example.getByRole("img")).toBeVisible();
  await expect(page.locator("iframe")).toHaveCount(0);
  await example.getByRole("tab", { name: "Rust 소스" }).click();
  await expect(example.getByRole("tabpanel").locator("pre")).toHaveText(
    await readFile(
      new URL("../../registries/neumorphism/items/chart-release-activity.rs", import.meta.url),
      "utf8",
    ),
  );
  await example.getByRole("button", { name: "설치 명령 복사" }).click();
  await expect
    .poll(() => page.evaluate(() => navigator.clipboard.readText()))
    .toBe(
      "gpui-deck add https://github.com/andongmin94/gpui-deck/blob/main/registries/neumorphism/chart-release-activity.json",
    );
  await filters.getByRole("link", { name: "누적 막대", exact: true }).click();
  await expect(page).toHaveURL(/\?type=stacked-bar$/);
  await expect(example).toHaveCount(1);
  await expect(example).toContainText("Neobrutal UI");
  await expect(example.getByRole("link", { name: "릴리스 활동", exact: true })).toHaveAttribute(
    "href",
    "/ko/directory/neobrutal#chart-release-activity",
  );
  await page.goBack();
  await expect(example).toContainText("Neumorphism UI");
  expect(await page.locator("html").evaluate((root) => root.scrollWidth <= innerWidth)).toBe(true);
  expect(pageErrors).toEqual([]);
});
