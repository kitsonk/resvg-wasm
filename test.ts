import { assertEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import { render } from "./mod.ts";

Deno.test({
  name: "basic",
  fn() {
    const data = render(
      `<?xml version="1.0" encoding="UTF-8"?>
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
      </svg>`,
    );
    assertEquals(data.byteLength, 9651);
  },
});
