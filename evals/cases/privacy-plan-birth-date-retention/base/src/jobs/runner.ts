/** Runs registered jobs once a day, at 06:00 server time. */
export type Job = { name: string; run: () => Promise<void> };

const jobs: Job[] = [];

export function daily(job: Job): void {
  jobs.push(job);
}

export async function runDaily(): Promise<void> {
  for (const job of jobs) {
    await job.run();
  }
}
