import { test, expect } from "@playwright/test";

test("homepage", async ({ page }) => {
  await page.goto("http://localhost:8080/");
});
