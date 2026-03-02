import { AnalysisResult } from "@/app/types/engine";

const ENGINE_URL = process.env.ENGINE_URL || "http://localhost:8080";

export async function analyzePosition(ply: string): Promise<AnalysisResult> {
  const node = ply.trim() === "" ? "root" : ply;

  const res = await fetch(`${ENGINE_URL}/analyze`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ node }),
  });

  if (!res.ok) {
    const body = await res.text();
    throw new Error(`Engine returned ${res.status}: ${body}`);
  }

  return res.json();
}
