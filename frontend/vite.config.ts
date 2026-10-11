import { defineConfig } from "vite";
import { tanstackRouter } from "@tanstack/router-plugin/vite";
import react from "@vitejs/plugin-react";

// akr-api on the host (`make run-api`). The liveness and readiness routes live outside /api/v1.
const api = "http://127.0.0.1:8080";
const proxy = { "/api": api, "/healthz": api, "/readyz": api };

// tanstackRouter() comes before react(): the route tree is generated from the source as written.
export default defineConfig({
  plugins: [tanstackRouter({ target: "react", autoCodeSplitting: true }), react()],
  server: { proxy },
  preview: { proxy },
});
