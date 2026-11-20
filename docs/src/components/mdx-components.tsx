import defaultMdxComponents from "fumadocs-ui/mdx";
import type { MDXComponents } from "mdx/types";

import { ComponentPreview } from "./preview";

const components = {
  ...defaultMdxComponents,
  ComponentPreview,
} satisfies MDXComponents;

export default components;
