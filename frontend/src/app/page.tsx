import { Button } from "@/components/ui/button";

export default function Home() {
  return (
    <main className="flex min-h-svh items-center justify-center p-6">
      <div className="w-full max-w-md space-y-6 text-center">
        <h1 className="text-3xl font-semibold tracking-tight">
          Habit Tracking
        </h1>
        <p className="text-muted-foreground">
          Your daily routines, habits, and tasks in one place.
        </p>
        <Button disabled>Today is coming soon</Button>
      </div>
    </main>
  );
}
