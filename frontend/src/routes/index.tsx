import { createFileRoute } from "@tanstack/react-router";

import { useHealthz } from "../api/generated/infra/infra";

export const Route = createFileRoute("/")({ component: Home });

function Home() {
  const health = useHealthz();
  return (
    <main>
      <h1>AKR-TMS</h1>
      <p>{health.data?.status}</p>
    </main>
  );
}
