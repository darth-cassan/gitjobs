import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: '.',
  failOnFlakyTests: true,
    use: {
    baseURL: process.env.BASE_URL || 'http://localhost:9000',
  },
  reporter: 'list',
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
  ],
});
