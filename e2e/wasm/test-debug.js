import { createEngine, initSync } from "kreuzcrawl";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const wasmPath = path.join(
  path.dirname(__filename),
  "./node_modules/kreuzcrawl/kreuzcrawl_wasm_bg.wasm",
);
const wasmBuffer = fs.readFileSync(wasmPath);
initSync({ module: wasmBuffer });

try {
  console.log("Testing createEngine(null)...");
  const engine = createEngine(null);
  console.log("Success! Engine created:", engine);
} catch (e) {
  console.log("Error:", e.message);
}
