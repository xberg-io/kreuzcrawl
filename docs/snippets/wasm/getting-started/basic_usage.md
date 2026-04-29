```javascript
import init, { createEngine, scrape } from "@kreuzberg/kreuzcrawl-wasm";

await init();

// Create engine with default settings
const engine = createEngine();

// Scrape a single page
const result = await scrape(engine, "https://example.com");
console.log(`Title: ${result.metadata.title}`);
console.log(`Status: ${result.statusCode}`);
console.log(`Links: ${result.links.length}`);
```
