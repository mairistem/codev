// Runs every migration up, down and up again against a throw-away database.
import { execSync } from "node:child_process";

for (const step of ["up", "down", "up"]) {
  execSync(`npx node-pg-migrate ${step} --database-url-var TEST_DATABASE_URL`, { stdio: "inherit" });
}
