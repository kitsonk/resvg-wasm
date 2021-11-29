import { render as wasmRender } from "./lib/resvg.generated.js";

/** Render an SVG string to a type array encoded as a PNG */
export function render(svg: string): Uint8Array {
  return wasmRender(svg);
}
