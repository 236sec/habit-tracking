import { NextResponse, type NextRequest } from "next/server";
import { appOrigin, isProvider } from "@/lib/supabase/config";
import { serverClient } from "@/lib/supabase/server";

export async function GET(
  request: NextRequest,
  context: { params: Promise<{ provider: string }> },
) {
  const { provider } = await context.params;
  if (!isProvider(provider))
    return NextResponse.redirect(new URL("/", request.url));
  const client = await serverClient(provider);
  const origin = appOrigin();
  if (!client || !origin)
    return NextResponse.redirect(new URL("/?error=configuration", request.url));
  // Only one provider session may be active in this browser at a time.
  const other = await serverClient(provider === "google" ? "github" : "google");
  if (other) await other.auth.signOut();
  await client.auth.signOut();
  const { data, error } = await client.auth.signInWithOAuth({
    provider,
    options: {
      redirectTo: new URL(
        `/auth/callback?provider=${provider}`,
        origin,
      ).toString(),
      skipBrowserRedirect: true,
    },
  });
  if (error || !data.url)
    return NextResponse.redirect(new URL("/?error=sign-in", request.url));
  return NextResponse.redirect(data.url);
}
