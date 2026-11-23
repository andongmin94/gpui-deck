import { expect, test } from "@playwright/test";

// Exercise the real WebAssembly preview on CI's software WebGPU adapter.
test.use({
  channel: "chromium",
  launchOptions: {
    args: [
      "--enable-unsafe-webgpu",
      "--enable-unsafe-swiftshader",
      "--use-webgpu-adapter=swiftshader",
      ...(process.platform === "linux"
        ? ["--enable-features=Vulkan", "--use-vulkan=swiftshader"]
        : []),
    ],
  },
});

test("login exposes named fields and reports the Rust form validation result", async ({ page }) => {
  await page.goto("/preview?c=login");
  const session = await page.context().newCDPSession(page);
  await session.send("Accessibility.enable");
  await expect
    .poll(async () => {
      const { nodes } = await session.send("Accessibility.getFullAXTree");
      return nodes
        .filter((node) => !node.ignored)
        .map((node) => `${node.role?.value}: ${node.name?.value}`);
    })
    .toEqual(
      expect.arrayContaining(["textbox: Email address", "textbox: Password", "button: Sign in"]),
    );

  await page.getByRole("button", { name: "Form controls" }).click();
  const email = page.getByRole("textbox", { name: "Email address" });
  const password = page.getByLabel("Password", { exact: true });
  await email.fill("sample@/");
  await password.fill("sample-password");
  await page.getByRole("button", { name: "Sign in" }).press("Enter");
  await expect(email).toHaveAttribute("aria-invalid", "true");
  await expect(page.getByText("Enter a valid email address.", { exact: true })).toBeVisible();

  await email.fill("sample@example.com");
  await password.press("Enter");
  await expect(page.getByRole("status")).toHaveText("Sample credentials validated locally.");
  await page.getByRole("button", { name: "Show canvas" }).click();
  await page.getByRole("button", { name: "Form controls" }).click();
  await expect(email).toHaveValue("sample@example.com");
});

test("settings exposes switch state and applies or discards edits", async ({ page }) => {
  await page.goto("/preview?c=sidebar-settings");
  await page.getByRole("button", { name: "Form controls" }).click();
  await page.getByRole("button", { name: "Appearance" }).click();
  const compact = page.getByRole("switch", { name: "Compact spacing" });
  const apply = page.getByRole("button", { name: "Apply changes" });
  const discard = page.getByRole("button", { name: "Discard changes" });
  await expect(compact).not.toBeChecked();
  await compact.press("Space");
  await expect(compact).toBeChecked();
  await apply.click();
  await expect(apply).toBeDisabled();
  await compact.press("Space");
  await expect(compact).not.toBeChecked();
  await discard.click();
  await expect(compact).toBeChecked();
  await expect(discard).toBeDisabled();
});

test("Neobrutal button shares activation between the canvas and keyboard controls", async ({
  page,
}) => {
  await page.goto("/preview?c=neobrutal-button");
  const canvas = page.getByRole("img", { name: "Neobrutal button preview" });
  await expect(canvas).toBeVisible();
  await canvas.click();
  await page.getByRole("button", { name: "Form controls" }).click();
  const button = page.getByRole("button", { name: "Pressed once" });
  await expect(button).toBeFocused();
  await button.press("Space");
  await expect(page.getByRole("button", { name: "Pressed 2 times" })).toBeFocused();
});

test("Neobrutal login validates the credentials entered through browser controls", async ({
  page,
}) => {
  await page.goto("/preview?c=neobrutal-login");
  await page.getByRole("button", { name: "Form controls" }).click();
  const email = page.getByRole("textbox", { name: "Email address" });
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(email).toHaveAttribute("aria-invalid", "true");
  await email.fill("designer@example.com");
  const password = page.getByLabel("Password", { exact: true });
  await password.fill("sample-password");
  await password.press("Enter");
  await expect(page.getByRole("status")).toHaveText("Sample credentials validated locally.");
});

test("Neobrutal tooltip paints its hint on hover and dismisses it with Escape", async ({
  page,
}) => {
  await page.setViewportSize({ width: 640, height: 400 });
  const errors: Error[] = [];
  page.on("pageerror", (error) => errors.push(error));
  await page.goto("/preview?c=neobrutal-tooltip");
  const canvas = page.locator("canvas");
  await expect(canvas).toBeVisible();

  // The WebGPU canvas exists before GPUI paints or hit-tests its first frame.
  const patch = (x: number, y: number) =>
    page.screenshot({ clip: { x, y, width: 50, height: 20 } });
  await expect
    .poll(async () => !(await patch(295, 190)).equals(await patch(15, 15)), { timeout: 15_000 })
    .toBe(true);
  await page.bringToFront();
  await canvas.click({ position: { x: 15, y: 15 } });

  // Mask the button's own hover feedback only while taking screenshots. The
  // tooltip can appear anywhere on the remaining canvas.
  const scene = () =>
    page.screenshot({
      style:
        "body::after { content: ''; position: fixed; inset: 50% auto auto 50%; transform: translate(-50%, -50%); width: 140px; height: 54px; background: black; pointer-events: none; z-index: 9999; }",
    });
  const closed = await scene();
  const hintPainted = async () => !(await scene()).equals(closed);
  await canvas.hover();
  await expect.poll(hintPainted, { timeout: 15_000 }).toBe(true);
  await page.keyboard.press("Escape");
  await expect.poll(hintPainted, { timeout: 10_000 }).toBe(false);
  expect(errors).toEqual([]);
});
