// Writes the host page that loads the wasm preview module into public/preview.
import { writeFileSync } from "node:fs";

writeFileSync(
  new URL("../public/preview/index.html", import.meta.url),
  `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>GPUI Design preview</title>
    <style>
      html, body { margin: 0; height: 100%; overflow: hidden; }
      body { background: #ffffff; }
      body.dark { background: #0a0a0a; }
      .fallback { font: 14px system-ui, sans-serif; color: #71717a; padding: 24px; }
    </style>
  </head>
  <body>
    <script type="module">
      const params = new URLSearchParams(location.search);
      if (params.get('theme') === 'dark') document.body.classList.add('dark');
      if (!('gpu' in navigator)) {
        document.body.innerHTML =
          '<p class="fallback">This preview needs a browser with WebGPU.</p>';
      } else {
        const { default: init } = await import('/preview/gpui_design_web_preview.js');
        await init();
      }
    </script>
  </body>
</html>
`,
);
