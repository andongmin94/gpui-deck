import { expect, test } from "@playwright/test";

test("the root opens the English home page", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveURL(/\/en\/?$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Build your component library for GPUI.",
  );
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
