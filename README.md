# resvg-wasm

Basic WASM bindings for [resvg](https://github.com/RazrFalcon/resvg) for Deno.

## render()

Take an SVG string and return a typed array with the data encoded as a PNG.

```ts
import { render } from "https://deno.land/x/resvg_wasm/mod.ts";

const data = render(`<?xml version="1.0" encoding="UTF-8"?>
<svg width="820px" height="312px" viewBox="0 0 820 312" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <title>Testing</title>
    <g id="testing" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
        <rect fill="#FFFFFF" x="0" y="0" width="820" height="312"></rect>
        <text id="test-text" font-family="sans-serif" font-size="32" font-weight="bold" fill="#111827">
            <tspan x="51" y="90">Testing Testing Testing</tspan>
        </text>
        <text id="monospace" font-family="monospace" font-size="32" font-weight="normal" fill="#2D53A4">
            <tspan x="502" y="233">Monospace</tspan>
        </text>
    </g>
</svg>`);

await Deno.writeFile("example.png", data);
```

## Built-in fonts

Currently there are three fonts built into the WASM library, used for rendering
text.

- `Bitter` is the default and the _serif_ font.
- `Inter` is the _sans-serif_ font.
- `JetBrains Mono` is the _monospace_ font.
