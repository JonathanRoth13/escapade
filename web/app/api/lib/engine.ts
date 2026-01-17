import { spawn, ChildProcess } from "child_process";
import { AnalysisResult } from "@/app/types/engine";

let engineProcess: ChildProcess | null = null;
let pendingResolve: ((value: string) => void) | null = null;

function initEngine() {
  const enginePath = process.env.ENGINE_PATH!;
  const tablebasePath = process.env.TABLEBASE_PATH;

  const args = ["engine"];
  if (tablebasePath) {
    args.push("--tablebase-dir", tablebasePath);
  }

  console.log("Starting engine:", enginePath, args);
  engineProcess = spawn(enginePath, args);

  let buffer = "";

  engineProcess.stdout!.on("data", (data) => {
    buffer += data.toString();
    const lines = buffer.split("\n");
    buffer = lines.pop() || "";

    for (const line of lines) {
      if (line.trim() && pendingResolve) {
        pendingResolve(line.trim());
        pendingResolve = null;
      }
    }
  });

  engineProcess.stderr!.on("data", (data) => {
    console.error("Engine stderr:", data.toString());
  });

  engineProcess.on("close", (code) => {
    console.log("Engine closed with code:", code);
    engineProcess = null;
  });

  engineProcess.on("error", (err) => {
    console.error("Engine error:", err);
    engineProcess = null;
  });
}

function getEngine() {
  if (!engineProcess) {
    initEngine();
  }
  return engineProcess;
}

// Start engine on module load
initEngine();

export async function analyzePosition(ply: string): Promise<AnalysisResult> {
  const command = `analyze "${ply}"`;
  const engine = getEngine();

  const result = await new Promise<string>((resolve) => {
    pendingResolve = resolve;
    (engine as ChildProcess).stdin!.write(command + "\n");
  });

  return JSON.parse(result);
}
