import { expect, test } from "@playwright/test";

test("component docs show themed GPUI captures without a WebGPU runtime", async ({ page }) => {
  await page.setViewportSize({ width: 360, height: 780 });
  await page.emulateMedia({ colorScheme: "dark" });
  await page.addInitScript(() => {
    Object.defineProperty(navigator, "gpu", { configurable: true, value: undefined });
    localStorage.setItem("gpui-deck-accent", "violet");
  });
  const runtime: string[] = [];
  page.on("request", (request) => {
    if (new URL(request.url()).pathname.startsWith("/preview")) runtime.push(request.url());
  });
  for (const [name, description] of [
    ["select", /서울이 선택/],
    ["dialog", /프로필 편집/],
    ["combobox", /SvelteKit이 강조/],
  ] as const) {
    const asset = `/images/components/${name}-violet-dark.webp`;
    const downloaded = page.waitForResponse(
      (response) => new URL(response.url()).pathname === asset,
    );
    await page.goto(`/ko/docs/components/${name}`);
    expect((await downloaded).ok()).toBe(true);
    const capture = page.getByRole("img", { name: description });
    await expect(capture).toBeVisible();
    await expect(capture).toHaveCSS("background-image", new RegExp(asset));
    await expect(page.getByText(/직접 조작하려면 WebGPU/)).toBeVisible();
    await expect(page.locator("iframe")).toHaveCount(0);
  }
  expect(runtime).toEqual([]);
});

test("a reader can launch the interactive component and return to the snapshot", async ({
  page,
}) => {
  await page.addInitScript(() => {
    Object.defineProperty(navigator, "gpu", {
      configurable: true,
      value: { requestAdapter: async () => ({ info: { isFallbackAdapter: false } }) },
    });
  });
  await page.route("**/preview?c=dialog", (route) =>
    route.fulfill({ contentType: "text/html", body: "<title>GPUI</title>" }),
  );
  await page.goto("/en/docs/components/dialog");
  const capture = page.getByRole("img", { name: /Edit profile dialog/ });
  await expect(capture).toBeVisible();
  await expect(page.locator("iframe")).toHaveCount(0);
  await page.getByRole("button", { name: "Run live preview" }).click();
  const frame = page.locator('iframe[title="dialog preview"]');
  await expect(frame.contentFrame().locator("title")).toHaveJSProperty("text", "GPUI");
  await page.getByRole("button", { name: "Back to snapshot" }).click();
  await expect(frame).toHaveCount(0);
  await expect(capture).toBeVisible();
});

test("an unavailable renderer leaves the capture visible without loading GPUI", async ({
  page,
}) => {
  await page.addInitScript(() => {
    Object.defineProperty(navigator, "gpu", {
      configurable: true,
      value: { requestAdapter: async () => null },
    });
    Object.defineProperty(HTMLCanvasElement.prototype, "getContext", { value: () => null });
  });
  await page.goto("/en/docs/components/select");
  await page.getByRole("button", { name: "Run live preview" }).click();
  await expect(page.getByRole("img", { name: /time-zone list/ })).toBeVisible();
  await expect(page.getByText(/browser with WebGPU to interact/)).toBeVisible();
  await expect(page.locator("iframe")).toHaveCount(0);
});
