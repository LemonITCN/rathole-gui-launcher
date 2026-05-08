import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "node:path";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    // Tauri's default targets are chrome105 (Win) and safari13 (mac/Linux),
    // but safari13 predates BigInt literals — antdv-next's deep deps ship
    // `0n` and esbuild can't lower it. Bumping the macOS / Linux baseline
    // to safari14 (≈ Big Sur, also the version of Safari macOS 10.15 ships
    // after auto-updates) lets BigInt pass through unchanged.
    target:
      process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari14",
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
}));
