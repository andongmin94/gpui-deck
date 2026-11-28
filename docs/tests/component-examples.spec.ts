import { readFile } from "node:fs/promises";
import { expect, test } from "@playwright/test";

test("a reader can copy the complete Rust example from the Korean Select guide", async ({
  context,
  page,
}) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.goto("/ko/docs/components/select");
  const article = page.getByRole("article");
  const source = article.locator("pre").filter({ hasText: "fn main()" });
  await expect(source).toBeVisible();
  await article.getByRole("button", { name: "코드 복사", exact: true }).click();
  const example = await readFile(
    new URL("../../crates/base/examples/select.rs", import.meta.url),
    "utf8",
  );
  await expect
    .poll(() =>
      page.evaluate(async () =>
        (await navigator.clipboard.readText()).replaceAll("\r\n", "\n").trimEnd(),
      ),
    )
    .toBe(example.replaceAll("\r\n", "\n").trimEnd());
});
