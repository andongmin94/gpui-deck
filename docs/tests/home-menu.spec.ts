import { expect, test } from "@playwright/test";

test("the mobile menu keeps the same destinations across the home, docs and blocks", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/ko");

  const header = page.getByRole("banner");
  await expect(header.getByRole("link", { name: "GPUI Deck" })).toBeVisible();
  const trigger = header.getByRole("button", { name: "탐색 열기/닫기" });
  const menu = page.getByRole("dialog", { name: "메뉴" });
  const nav = menu.getByRole("navigation", { name: "메뉴", exact: true });
  await expect(async () => {
    await trigger.press("Enter");
    await expect(menu).toBeVisible({ timeout: 1000 });
  }).toPass();
  await expect(nav.getByRole("link")).toHaveText([
    "홈",
    "문서",
    "컴포넌트",
    "블록",
    "차트",
    "레시피",
    "레지스트리",
    "타이포그래피",
    "만들기",
  ]);
  await expect(menu.getByRole("link", { name: "설치", exact: true })).toHaveAttribute(
    "href",
    "/ko/docs/installation",
  );
  await page.keyboard.press("Escape");
  await expect(menu).toBeHidden();
  await expect(trigger).toBeFocused();

  await trigger.press("Enter");
  await nav.getByRole("link", { name: "레시피" }).press("Enter");
  await expect(page).toHaveURL(/\/ko\/docs\/recipes\/?$/);
  await expect(menu).toBeHidden();

  await trigger.click();
  await expect(menu.getByRole("navigation", { name: "문서", exact: true })).toBeVisible();
  await nav.getByRole("link", { name: "블록", exact: true }).click();
  await expect(page).toHaveURL(/\/ko\/blocks\/?$/);
  await trigger.click();
  await menu.getByRole("link", { name: "블록 공유하기", exact: true }).click();
  await expect(page).toHaveURL(/\/ko\/docs\/guides\/blocks\/?#publish$/);
  await expect(page.getByRole("heading", { name: /우리 팀의 블록 공유/ })).toBeInViewport();
});
