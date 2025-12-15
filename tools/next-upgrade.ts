import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

function fail(message: string): never {
  console.error(message);
  process.exit(1);
}

const requested = process.argv.slice(2).find((arg) => !arg.startsWith("-"));
if (!requested) {
  fail(
    "Usage: pnpm next:upgrade -- <nextVersion>\nExample: pnpm next:upgrade -- 15.5.7",
  );
}

const root = process.cwd();
const packageJsonPath = path.join(root, "package.json");

const raw = fs.readFileSync(packageJsonPath, "utf8");
const parsed = JSON.parse(raw) as any;

parsed.pnpm ??= {};
parsed.pnpm.overrides ??= {};

parsed.pnpm.overrides.next = requested;

fs.writeFileSync(packageJsonPath, JSON.stringify(parsed, null, 2) + "\n", "utf8");
console.log(`Updated pnpm.overrides.next -> ${requested}`);

const run = (command: string, args: string[]) => {
  const result = spawnSync(command, args, {
    stdio: "inherit",
    shell: process.platform === "win32",
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
};

run("pnpm", ["install"]);
run("pnpm", ["-r", "build"]);
