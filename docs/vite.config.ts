import tailwindcss from "@tailwindcss/vite";
import { fumadocsMdx } from "fumadocs-mdx/vite";
import press from "fumapress/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    press({ components: "./src/components/mdx-components.tsx" }),
    fumadocsMdx(),
    tailwindcss(),
  ],
  resolve: {
    tsconfigPaths: true,
  },
});
