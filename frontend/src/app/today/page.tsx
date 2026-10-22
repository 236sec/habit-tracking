import { redirect } from "next/navigation";
import { connection } from "next/server";
import { activeSession } from "@/lib/supabase/session";

export const instant = false;

type MeResponse = { success: boolean; data?: { id: string } };

export default async function Today() {
  await connection();
  const session = await activeSession();
  if (!session) redirect("/");
  const base = process.env.BACKEND_URL;
  if (!base) throw new Error("BACKEND_URL must be set");
  const response = await fetch(`${base.replace(/\/$/, "")}/api/me`, {
    headers: { Authorization: `Bearer ${session.token}` },
    cache: "no-store",
  });
  if (response.status === 401) redirect("/");
  if (!response.ok) throw new Error("Account service unavailable");
  const me = (await response.json()) as MeResponse;
  if (!me.success || !me.data) throw new Error("Account service unavailable");

  return (
    <main className="min-h-svh bg-stone-50">
      <div className="mx-auto max-w-5xl px-5 py-6 sm:px-8 sm:py-10">
        <header className="flex items-center justify-between gap-4">
          <div>
            <span className="text-sm font-semibold tracking-wide text-emerald-700">
              HABIT TRACKING
            </span>
            <p className="text-xs text-stone-500">
              Signed in with{" "}
              {session.provider === "google" ? "Google" : "GitHub"}
            </p>
          </div>
          <form action="/auth/sign-out" method="post">
            <button
              className="rounded-lg border border-stone-300 bg-white px-4 py-2 text-sm font-medium hover:bg-stone-100"
              type="submit"
            >
              Sign out
            </button>
          </form>
        </header>
        <div className="mt-12 sm:mt-16">
          <p className="text-sm font-medium text-emerald-700">
            YOUR DAILY SPACE
          </p>
          <h1 className="mt-2 text-4xl font-semibold tracking-tight text-stone-900 sm:text-5xl">
            Today
          </h1>
          <p className="mt-4 max-w-xl text-stone-600">
            A fresh start for the things that matter to you.
          </p>
        </div>
        <div className="mt-10 grid gap-4 md:grid-cols-3">
          <section className="rounded-2xl border border-stone-200 bg-white p-6">
            <h2 className="text-lg font-semibold">Morning routine</h2>
            <p className="mt-3 text-sm text-stone-600">
              Your morning routine will appear here.
            </p>
          </section>
          <section className="rounded-2xl border border-stone-200 bg-white p-6">
            <h2 className="text-lg font-semibold">Habits</h2>
            <p className="mt-3 text-sm text-stone-600">
              Your planned habits will appear here.
            </p>
          </section>
          <section className="rounded-2xl border border-stone-200 bg-white p-6">
            <h2 className="text-lg font-semibold">Tasks</h2>
            <p className="mt-3 text-sm text-stone-600">
              Due and overdue tasks will appear here.
            </p>
          </section>
        </div>
      </div>
    </main>
  );
}
