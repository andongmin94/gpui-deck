import { expect, test, type Page } from "@playwright/test";

async function openResult(page: Page, query: string, title: string) {
  const dialog = page.getByRole("dialog");
  const input = dialog.getByRole("combobox");
  await expect(async () => {
    if (!(await dialog.isVisible())) await page.keyboard.press("ControlOrMeta+k");
    await expect(input).toBeVisible({ timeout: 1000 });
  }).toPass();
  await input.fill(query);
  await expect(dialog.getByRole("option").first()).toContainText(title);
  await input.press("Enter");
  await expect(dialog).toBeHidden();
}

test("global search opens named blocks and selects individual registry charts", async ({
  page,
}) => {
  await page.goto("/ko");
  await openResult(page, "청구 대시보드", "청구 대시보드");
  await expect(page).toHaveURL(/\/ko\/blocks\/billing-dashboard$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("청구 대시보드");

  await openResult(page, "Work Allocation", "작업 배분");
  await expect(page).toHaveURL(/\/ko\/directory\/neobrutal#chart-work-allocation$/);
  await expect(page.locator("#registry-item-title")).toHaveText("작업 배분");
  await openResult(page, "neobrutal-chart-release-activity", "릴리스 활동");
  await expect(page).toHaveURL(/\/ko\/directory\/neobrutal#chart-release-activity$/);
  await expect(page.locator("#registry-item-title")).toHaveText("릴리스 활동");
  await openResult(page, "세이지 프리셋", "세이지");
  await expect(page).toHaveURL(/\/ko\/directory\/neumorphism#style-sage$/);
  await expect(page.locator("#registry-item-title")).toHaveText("세이지");
});

test("mobile global search applies successive named presets in Create", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/ko/create");
  const style = page.getByRole("combobox", { name: "스타일", exact: true });
  await openResult(page, "뉴모피즘", "뉴모피즘");
  await expect(page).toHaveURL(/\/ko\/create\?preset=v1\.neumorphism\./);
  await expect(style).toHaveValue("neumorphism");
  await openResult(page, "네오브루탈", "네오브루탈");
  await expect(page).toHaveURL(/\/ko\/create\?preset=v1\.neobrutal\./);
  await expect(style).toHaveValue("neobrutal");
  await style.selectOption("base");
  await openResult(page, "Neobrutal", "네오브루탈");
  await expect(style).toHaveValue("neobrutal");
});

test("compare compact mobile blocks and find installable screens by feature", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/ko/blocks?purpose=authentication");
  const cards = page.getByRole("article");
  await expect(cards).toHaveCount(2);
  await expect(cards.last().locator("p")).toBeInViewport({ ratio: 1 });
  await expect(page.getByRole("tab")).toHaveCount(0);
  await cards.first().getByRole("button", { name: "미리보기" }).click();
  await expect(cards.first().getByRole("tab", { name: "Rust 소스" })).toBeVisible();
  await cards.last().getByRole("button", { name: "미리보기" }).click();
  await expect(cards.first().getByRole("tab")).toHaveCount(0);
  await expect(cards.last().getByRole("tab", { name: "Rust 소스" })).toBeVisible();

  await page.getByRole("combobox", { name: "용도" }).selectOption("management");
  const search = page.getByRole("searchbox", { name: "블록 검색" });
  await search.fill("초안");
  await expect(cards).toHaveCount(3);
  await page.getByRole("combobox", { name: "스타일" }).selectOption("neumorphism");
  await expect(cards).toHaveCount(2);
  await search.fill("일괄 삭제");
  await expect(cards).toHaveCount(1);
  await expect(cards.getByRole("heading")).toContainText("데이터 관리");
  await page.reload();
  await expect(search).toHaveValue("일괄 삭제");
  await expect(cards).toHaveCount(1);
  await openResult(page, "레코드 편집", "데이터 관리");
  await expect(page).toHaveURL(/\/ko\/directory\/neumorphism#template-data-manager$/);
});
