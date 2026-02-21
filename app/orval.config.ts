import { defineConfig } from "orval";

export default defineConfig({
  petstore: {
    input: "http://localhost:3000/openapi.json",
    output: {
      target: "./src/lib/api.ts",
      client: "svelte-query",
      baseUrl: "http://localhost:3000",
      biome: true
    }
  }
});
