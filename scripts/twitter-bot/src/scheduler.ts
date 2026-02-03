import cron from "node-cron";

export type SchedulerCallbacks = {
  onDailyRecap: () => Promise<void>;
  onCountdownAlert: () => Promise<void>;
};

export const startScheduler = (callbacks: SchedulerCallbacks) => {
  // Daily recap at midnight UTC
  cron.schedule("0 0 * * *", () => {
    callbacks.onDailyRecap().catch((err) => console.error("Daily recap failed", err));
  }, { timezone: "UTC" });
};

export const scheduleCountdownAlert = (
  interrogationAt: Date,
  callbacks: Pick<SchedulerCallbacks, "onCountdownAlert">
) => {
  const alertTime = new Date(interrogationAt.getTime() - 60 * 60 * 1000);
  const delay = alertTime.getTime() - Date.now();
  if (delay <= 0) {
    console.log("Countdown alert time already passed, skipping.");
    return;
  }
  setTimeout(() => {
    callbacks.onCountdownAlert().catch((err) => console.error("Countdown alert failed", err));
  }, delay);
};
