import { readFile } from "node:fs/promises";
import { expect, test } from "@playwright/test";

const sage = "v1.neumorphism.sage.sage.forest.pretendard.pretendard.outline.small";

test.use({
  channel: "chromium",
  launchOptions: {
    args: [
      "--enable-unsafe-webgpu",
      "--enable-unsafe-swiftshader",
      "--use-webgpu-adapter=swiftshader",
      "--use-angle=swiftshader",
    ],
  },
});

test("a shared preset renders in GPUI and exports the edited selection", async ({
  page,
  context,
}) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.goto(`/en/create?preset=${sage}`);
  await expect(page.getByRole("combobox", { name: "Style", exact: true })).toHaveValue(
    "neumorphism",
  );
  const frame = page.locator('iframe[title="Live GPUI preview"]');
  await expect(frame).toHaveAttribute("data-preset", sage);
  const canvas = frame.contentFrame().locator("canvas");
  await expect(canvas).toBeVisible({ timeout: 20_000 });
  const original = await frame.evaluateHandle(
    (element: HTMLIFrameElement) => element.contentDocument,
  );
  const before = await canvas.screenshot();
  await page.getByRole("combobox", { name: "Base color", exact: true }).selectOption("lavender");
  const changed = sage.replace("neumorphism.sage", "neumorphism.lavender");
  await expect(frame).toHaveAttribute("data-preset", changed);
  await expect.poll(async () => !(await canvas.screenshot()).equals(before)).toBe(true);
  expect(
    await frame.evaluate(
      (element: HTMLIFrameElement, doc) => element.contentDocument === doc,
      original,
    ),
  ).toBe(true);

  await page.getByRole("button", { name: "Share", exact: true }).click();
  await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(page.url());
  await page.getByRole("button", { name: "Get code", exact: true }).click();
  const dialog = page.getByRole("dialog", { name: "Get code" });
  await dialog.getByRole("button", { name: "Copy install command" }).click();
  await expect
    .poll(() => page.evaluate(() => navigator.clipboard.readText()))
    .toBe(`gpui-deck add --preset ${changed}`);
  await dialog.getByRole("button", { name: "Copy Rust code" }).click();
  await expect
    .poll(() =>
      page.evaluate(async () => (await navigator.clipboard.readText()).replaceAll("\r\n", "\n")),
    )
    .toBe(`pub const CODE: &str = "${changed}";\n`);
  await dialog.getByRole("combobox", { name: "Source file" }).selectOption("showcase.rs");
  await expect(dialog.locator("pre code").filter({ hasText: "pub struct Showcase" })).toHaveText(
    await readFile(new URL("../../crates/cli/presets/create/showcase.rs", import.meta.url), "utf8"),
  );
  await dialog.getByRole("combobox", { name: "Source file" }).selectOption("gallery.rs");
  await expect(dialog.locator("pre", { hasText: "pub struct Gallery" })).toHaveText(
    await readFile(new URL("../../crates/web-preview/src/gallery.rs", import.meta.url), "utf8"),
  );
  await expect(dialog.getByText("let view = ui::gallery::open(false, window, cx);")).toBeVisible();
});

test("mobile users can open a preset, shuffle and reset without graphics support", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.addInitScript(() => Object.defineProperty(navigator, "gpu", { value: undefined }));
  await page.goto("/en/create?preset=unknown");
  await expect(page.getByRole("alert")).toContainText("invalid preset");
  await expect(page.locator("iframe")).toHaveCount(0);
  await page.getByRole("button", { name: "Customize" }).click();
  await page.getByRole("button", { name: "Open preset" }).click();
  const dialog = page.getByRole("dialog", { name: "Open preset" });
  await dialog.getByRole("textbox").fill("v2.unrecognized");
  await dialog.getByRole("button", { name: "Open", exact: true }).click();
  await expect(dialog.getByRole("textbox")).toHaveAttribute("aria-invalid", "true");
  await dialog
    .getByRole("textbox")
    .fill(`https://gpui-deck.andongmin.com/ko/create?preset=${sage}`);
  await dialog.getByRole("button", { name: "Open", exact: true }).click();
  await expect(dialog).toBeHidden();
  await expect(page.getByRole("combobox", { name: "Base color" })).toHaveValue("sage");
  await page.evaluate(() => {
    Math.random = () => 0.5;
  });
  await page.getByRole("button", { name: "Shuffle", exact: true }).click();
  await expect.poll(() => new URL(page.url()).searchParams.get("preset")).not.toBe(sage);
  await page.goBack();
  await expect(page.getByRole("combobox", { name: "Style", exact: true })).toHaveValue(
    "neumorphism",
  );
  await page.getByRole("button", { name: "Reset", exact: true }).click();
  await expect(page).toHaveURL(/\/en\/create\/?$/);
  await expect(page.getByRole("combobox", { name: "Body font" })).toHaveValue("pretendard");
  expect(await page.locator("html").evaluate((root) => root.scrollWidth <= innerWidth)).toBe(true);
});
