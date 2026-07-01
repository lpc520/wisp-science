import { defineConfig } from "@playwright/test";

// E2E for the wisp-science Leptos UI. A `trunk serve` dev server hosts the frontend
// at :1420; Playwright injects a mocked `window.__TAURI__` before the page
// loads so the UI's invoke/listen calls resolve against canned data — no Rust
// backend or API key required.
export default defineConfig({
  testDir: "./tests",
  timeout: 30_000,
  expect: { timeout: 10_000 },
  fullyParallel: false,
  use: {
    baseURL: "http://localhost:1420",
    browserName: "chromium",
    trace: "on-first-retry",
  },
  webServer: {
    command: "trunk serve --port 1420",
    url: "http://localhost:1420",
    reuseExistingServer: true,
    timeout: 180_000,
    cwd: "../ui",
  },
});
