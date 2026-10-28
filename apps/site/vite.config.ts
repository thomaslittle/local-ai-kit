import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Static, self-contained build: no server functions, no remote runtime calls.
export default defineConfig({
  plugins: [react()],
  build: {
    target: "es2020",
    sourcemap: true,
    assetsInlineLimit: 2048,
    rolldownOptions: {
      output: {
        codeSplitting: {
          groups: [
            { name: "three-core", test: /node_modules[\\/]three[\\/]/ },
            { name: "react-vendor", test: /node_modules[\\/](react|react-dom|scheduler)[\\/]/ },
            { name: "postproc", test: /node_modules[\\/]postprocessing[\\/]/ },
            { name: "fiber", test: /node_modules[\\/]@react-three[\\/]fiber[\\/]/ },
            { name: "drei", test: /node_modules[\\/]@react-three[\\/]drei[\\/]/ },
          ],
        },
      },
    },
  },
});
