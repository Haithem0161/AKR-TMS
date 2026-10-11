import { defineConfig } from "orval";

// The client is generated from the document `make generate-api` writes, never from a running server,
// so the committed client and the committed document always belong together.
export default defineConfig({
  akr: {
    input: { target: "./openapi.json" },
    output: {
      mode: "tags-split",
      target: "src/api/generated",
      schemas: "src/api/models",
      client: "react-query",
      httpClient: "axios",
      headers: true,
      clean: true,
      formatter: "prettier",
      override: {
        mutator: { path: "src/api/axios-instance.ts", name: "customInstance" },
      },
    },
  },
});
