import { NextResponse, type NextRequest } from "next/server";
import { serverClient } from "@/lib/supabase/server";

export async function POST(request: NextRequest) {
  for (const provider of ["google", "github"] as const) {
    const client = await serverClient(provider);
    if (client) await client.auth.signOut();
  }
  return NextResponse.redirect(new URL("/", request.url), 303);
}
