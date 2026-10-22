import { createServerClient } from "@supabase/ssr";
import { NextResponse, type NextRequest } from "next/server";
import { providerConfig } from "@/lib/supabase/config";

export async function proxy(request: NextRequest) {
  let response = NextResponse.next({ request });
  for (const provider of ["google", "github"] as const) {
    const config = providerConfig(provider);
    if (!config) continue;
    const client = createServerClient(config.url, config.key, {
      cookieOptions: { name: config.cookieName },
      cookies: {
        getAll() {
          return request.cookies.getAll();
        },
        setAll(cookiesToSet) {
          for (const { name, value } of cookiesToSet)
            request.cookies.set(name, value);
          response = NextResponse.next({ request });
          for (const { name, value, options } of cookiesToSet)
            response.cookies.set(name, value, options);
        },
      },
    });
    await client.auth.getUser();
  }
  return response;
}

export const config = { matcher: ["/today/:path*", "/auth/:path*"] };
