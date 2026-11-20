import { expect, test } from "@playwright/test";

test("the root opens the English home page", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveURL(/\/en\/?$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Build your component library for GPUI.",
  );
});

test("the Korean home page links into the Korean docs", async ({ page }) => {
  await page.goto("/ko");
  await expect(page.locator("html")).toHaveAttribute("lang", "ko");
  await page.getByRole("link", { name: "시작하기" }).click();
  await expect(page).toHaveURL(/\/ko\/docs\/?$/);
});

test("a component page embeds its live preview", async ({ page }) => {
  await page.goto("/en/docs/components/button");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Button");
  await expect(page.locator('iframe[title="button preview"]')).toHaveAttribute(
    "src",
    /\/preview\/\?c=button&theme=(light|dark)$/,
  );
});

test("a Korean page links to its English original", async ({ page }) => {
  await page.goto("/ko/docs/components/button");
  await expect(page.getByRole("link", { name: "영어 원문 보기" })).toHaveAttribute(
    "href",
    "/en/docs/components/button",
  );
});

test("search finds a component page", async ({ page }) => {
  await page.goto("/en/docs");
  await page
    .getByRole("button", { name: /search/i })
    .first()
    .click();
  await page.getByRole("dialog").getByRole("combobox").fill("carousel");
  await expect(page.getByRole("dialog").getByText("Carousel").first()).toBeVisible();
});
