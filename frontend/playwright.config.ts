import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  use: { baseURL: "http://localhost:3100" },
  webServer: [
    {
      command: "node tests/support/mock-auth.mjs",
      url: "http://127.0.0.1:4101/health",
      reuseExistingServer: !process.env.CI,
    },
    {
      command: "npm run dev -- --hostname 127.0.0.1 --port 3100",
      url: "http://127.0.0.1:3100",
      reuseExistingServer: !process.env.CI,
      timeout: 60_000,
      env: {
        NEXT_PUBLIC_GOOGLE_SUPABASE_URL: "http://127.0.0.1:4101",
        NEXT_PUBLIC_GOOGLE_SUPABASE_PUBLISHABLE_KEY: "test-google-key",
        NEXT_PUBLIC_GITHUB_SUPABASE_URL: "http://127.0.0.1:4102",
        NEXT_PUBLIC_GITHUB_SUPABASE_PUBLISHABLE_KEY: "test-github-key",
        BACKEND_URL: "http://127.0.0.1:4103",
        APP_ORIGIN: "http://localhost:3100",
      },
    },
  ],
});
