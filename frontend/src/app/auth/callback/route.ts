import { NextResponse, type NextRequest } from "next/server";
import { isProvider } from "@/lib/supabase/config";
import { serverClient } from "@/lib/supabase/server";

export async function GET(request: NextRequest) {
  const provider = request.nextUrl.searchParams.get("provider") ?? "";
  const code = request.nextUrl.searchParams.get("code");
  if (!isProvider(provider) || !code)
    return NextResponse.redirect(new URL("/?error=callback", request.url));
  const client = await serverClient(provider);
  if (!client)
    return NextResponse.redirect(new URL("/?error=configuration", request.url));
  const { error } = await client.auth.exchangeCodeForSession(code);
  return NextResponse.redirect(
    new URL(error ? "/?error=callback" : "/today", request.url),
  );
}
