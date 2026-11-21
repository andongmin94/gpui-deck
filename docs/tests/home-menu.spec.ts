import { expect, test } from "@playwright/test";

test("the mobile home menu supports keyboard navigation to localized docs", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/ko");

  const header = page.getByRole("banner");
  await expect(header.getByRole("link", { name: "GPUI Deck" })).toBeVisible();
  const trigger = header.getByRole("button", { name: "탐색 열기/닫기" });
  const menu = page.getByRole("menu");
  await expect(async () => {
    await trigger.press("ArrowDown");
    await expect(menu).toBeVisible({ timeout: 1000 });
  }).toPass();
  await expect(menu.getByRole("menuitem")).toHaveText([
    "문서",
    "컴포넌트",
    "레시피",
    "블록",
    "가이드",
  ]);
  await page.keyboard.press("Escape");
  await expect(menu).toBeHidden();
  await expect(trigger).toBeFocused();

  await trigger.press("ArrowDown");
  await expect(menu.getByRole("menuitem", { name: "문서" })).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await expect(menu.getByRole("menuitem", { name: "레시피" })).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(page).toHaveURL(/\/ko\/docs\/recipes\/?$/);

  await trigger.click();
  await expect(page.getByRole("navigation", { name: "문서" })).toBeVisible();
});
