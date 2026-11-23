import { readFile } from "node:fs/promises";
import { expect, test } from "@playwright/test";

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
  for (const title of ["Login", "Button", "Input"]) {
    const slug = title.toLowerCase();
    const item = page.getByRole("region", { name: title, exact: true });
    const command = `gpui-deck add https://github.com/andongmin94/gpui-deck/blob/main/registries/neobrutal/${slug}.json`;
    await expect(item.getByRole("textbox", { name: "Install command" })).toHaveValue(command);
    await item.getByRole("button", { name: "Copy install command" }).click();
    await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(command);
    await item.getByRole("tab", { name: "Rust source" }).click();
    await item.getByRole("combobox", { name: "Source file" }).selectOption(`${slug}.rs`);
    await expect(item.locator("pre")).toHaveText(
      await readFile(new URL(`${slug}.rs`, directory), "utf8"),
    );
  }
  expect(runtime).toEqual([]);
});
