import { NextRequest, NextResponse } from "next/server";
import { analyzePosition } from "../lib/engine";

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams;
  const ply = searchParams.get("ply");

  if (!ply) {
    return NextResponse.json(
      { error: "Missing required parameter: ply" },
      { status: 400 },
    );
  }

  const analysis = await analyzePosition(ply);

  if (!analysis) {
    return NextResponse.json(
      { error: "Engine returned empty response" },
      { status: 500 },
    );
  }

  return NextResponse.json(analysis);
}
