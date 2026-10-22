/* eslint-disable @next/next/no-html-link-for-pages -- OAuth sign-in needs a full browser navigation. */
import { connection } from "next/server";
import { redirect } from "next/navigation";
import { activeSession } from "@/lib/supabase/session";

export const instant = false;

export default async function Home({
  searchParams,
}: {
  searchParams: Promise<{ error?: string }>;
}) {
  await connection();
  if (await activeSession()) redirect("/today");
  const { error } = await searchParams;
  return (
    <main className="flex min-h-svh items-center justify-center bg-stone-50 px-5 py-10">
      <section className="w-full max-w-md rounded-3xl border border-stone-200 bg-white p-7 shadow-sm sm:p-10">
        <p className="mb-5 text-sm font-semibold tracking-wide text-emerald-700">
          HABIT TRACKING
        </p>
        <h1 className="text-3xl font-semibold tracking-tight text-stone-900">
          Make today count.
        </h1>
        <p className="mt-3 text-stone-600">
          Keep your morning routine, habits, and priorities in one calm place.
        </p>
        {error && (
          <p
            role="alert"
            className="mt-5 rounded-xl bg-rose-50 p-3 text-sm text-rose-800"
          >
            Sign-in could not be completed. Check your account setup and try
            again.
          </p>
        )}
        <div className="mt-8 grid gap-3">
          <a
            className="flex min-h-12 items-center justify-center rounded-xl bg-stone-900 px-4 font-medium text-white hover:bg-stone-700"
            href="/auth/sign-in/google"
          >
            Continue with Google
          </a>
          <a
            className="flex min-h-12 items-center justify-center rounded-xl border border-stone-300 px-4 font-medium text-stone-900 hover:bg-stone-50"
            href="/auth/sign-in/github"
          >
            Continue with GitHub
          </a>
        </div>
        <p className="mt-6 text-center text-sm text-stone-500">
          Your daily plan will be here when you return.
        </p>
      </section>
    </main>
  );
}
