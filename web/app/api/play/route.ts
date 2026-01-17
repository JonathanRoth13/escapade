import { NextRequest, NextResponse } from "next/server";
import { analyzePosition } from "../lib/engine";
import { processAnalysis } from "./processor";

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams;
  const ply = searchParams.get("ply");
  const strengthParam = searchParams.get("strength");

  if (!ply) {
    return NextResponse.json(
      { error: "Missing required parameter: ply" },
      { status: 400 },
    );
  }

  const strength = strengthParam ? parseInt(strengthParam, 10) : 100;

  const analysis = await analyzePosition(ply);

  if (!analysis) {
    return NextResponse.json(
      { error: "Engine returned empty response" },
      { status: 500 },
    );
  }

  const response = await processAnalysis(strength, analysis, analyzePosition);

  return NextResponse.json(response);
}
