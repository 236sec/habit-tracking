import { createServerClient } from "@supabase/ssr";
import { cookies } from "next/headers";
import { providerConfig, type Provider } from "./config";

export async function serverClient(provider: Provider) {
  const config = providerConfig(provider);
  if (!config) return null;
  const store = await cookies();
  return createServerClient(config.url, config.key, {
    cookieOptions: { name: config.cookieName },
    cookies: {
      getAll() {
        return store.getAll();
      },
      setAll(cookiesToSet) {
        for (const { name, value, options } of cookiesToSet) {
          try {
            store.set(name, value, options);
          } catch {
            /* Server Component: proxy refreshes cookies. */
          }
        }
      },
    },
  });
}
