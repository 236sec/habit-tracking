export type Provider = "google" | "github";

export function providerConfig(provider: Provider) {
  const prefix = provider === "google" ? "GOOGLE" : "GITHUB";
  const url = process.env[`NEXT_PUBLIC_${prefix}_SUPABASE_URL`];
  const key = process.env[`NEXT_PUBLIC_${prefix}_SUPABASE_PUBLISHABLE_KEY`];
  if (!url || !key) return null;
  return { url, key, cookieName: `habit-auth-${provider}` };
}

export function isProvider(value: string): value is Provider {
  return value === "google" || value === "github";
}

export function appOrigin() {
  const value = process.env.APP_ORIGIN;
  if (!value) return null;
  const url = new URL(value);
  if (url.pathname !== "/" || url.search || url.hash) return null;
  return url.origin;
}
