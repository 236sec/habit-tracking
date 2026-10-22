import { serverClient } from "./server";
import type { Provider } from "./config";

export async function activeSession() {
  const found: { provider: Provider; token: string }[] = [];
  for (const provider of ["google", "github"] as const) {
    const client = await serverClient(provider);
    if (!client) continue;
    const {
      data: { user },
      error,
    } = await client.auth.getUser();
    if (error || !user) continue;
    const {
      data: { session },
    } = await client.auth.getSession();
    if (session?.access_token)
      found.push({ provider, token: session.access_token });
  }
  return found.length === 1 ? found[0] : null;
}
