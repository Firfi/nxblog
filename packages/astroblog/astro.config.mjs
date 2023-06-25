import { defineConfig } from 'astro/config';
import react from "@astrojs/react";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://astro.build/config
export default defineConfig({
  outDir: '../../dist/packages/astroblog',
  integrations: [react()],
  vite: {
    plugins: [
      wasm()//, topLevelAwait()
    ]
  }
  // root: '../../dist/packages/astroblog'
});
