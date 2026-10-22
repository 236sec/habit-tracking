import { test, expect } from "@playwright/test";

for (const width of [390, 1280]) {
  test(`signed-out visitor sees both provider choices and cannot open Today at ${width}px`, async ({
    page,
  }) => {
    await page.setViewportSize({ width, height: 850 });
    await page.goto("/");
    await expect(
      page.getByRole("heading", { name: "Make today count." }),
    ).toBeVisible();
    await expect(
      page.getByRole("link", { name: "Continue with Google" }),
    ).toHaveAttribute("href", "/auth/sign-in/google");
    await expect(
      page.getByRole("link", { name: "Continue with GitHub" }),
    ).toHaveAttribute("href", "/auth/sign-in/github");
    await page.goto("/today");
    await expect(page).toHaveURL("/");
    await expect(
      page.getByRole("heading", { name: "Make today count." }),
    ).toBeVisible();
  });

  test(`same-email providers retain separate accounts through sign-out and return at ${width}px`, async ({
    page,
  }) => {
    await page.setViewportSize({ width, height: 850 });
    await page.goto("/");
    await page.getByRole("link", { name: "Continue with Google" }).click();
    await expect(page).toHaveURL("/today");
    await expect(
      page.getByText("Signed in with Google · Account 11111111"),
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Morning routine" }),
    ).toBeVisible();
    await expect(page.getByRole("heading", { name: "Habits" })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Tasks" })).toBeVisible();
    await page.reload();
    await expect(
      page.getByText("Signed in with Google · Account 11111111"),
    ).toBeVisible();
    await page.getByRole("button", { name: "Sign out" }).click();
    await expect(page).toHaveURL("/");
    await page.getByRole("link", { name: "Continue with GitHub" }).click();
    await expect(page).toHaveURL("/today");
    await expect(
      page.getByText("Signed in with GitHub · Account 22222222"),
    ).toBeVisible();
    await page.getByRole("button", { name: "Sign out" }).click();
    await page.getByRole("link", { name: "Continue with Google" }).click();
    await expect(
      page.getByText("Signed in with Google · Account 11111111"),
    ).toBeVisible();
  });
}
