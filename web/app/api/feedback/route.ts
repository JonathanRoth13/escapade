import { NextRequest, NextResponse } from "next/server";
import { Resend } from "resend";

/**
 * POST /api/feedback
 * Body: { message: string; name?: string; contact?: string }
 */

const MAX = { message: 5000, name: 200, contact: 200 };

const RATE_LIMIT = { count: 5, windowMs: 10 * 60 * 1000, maxSenders: 1000 };
const recent = new Map<string, number[]>();

function rateLimited(ip: string): boolean {
  const now = Date.now();
  if (!recent.has(ip) && recent.size >= RATE_LIMIT.maxSenders) {
    for (const [key, times] of recent) {
      if (times.every((t) => now - t >= RATE_LIMIT.windowMs))
        recent.delete(key);
    }
    // Still full: refuse new senders so the map can't grow without bound.
    if (recent.size >= RATE_LIMIT.maxSenders) return true;
  }
  const times = (recent.get(ip) ?? []).filter(
    (t) => now - t < RATE_LIMIT.windowMs,
  );
  if (times.length >= RATE_LIMIT.count) {
    recent.set(ip, times);
    return true;
  }
  recent.set(ip, [...times, now]);
  return false;
}

const field = (value: unknown, max: number) =>
  typeof value === "string" ? value.trim().slice(0, max) : "";

const oneLine = (s: string) => s.replace(/[\p{Cc}\p{Zl}\p{Zp}]+/gu, " ").trim();

const looksLikeEmail = (s: string) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(s);

export async function POST(request: NextRequest) {
  const type = request.headers.get("content-type")?.toLowerCase() ?? "";
  if (!type.startsWith("application/json")) {
    return new NextResponse(null, { status: 415 });
  }

  let body: Record<string, unknown>;
  try {
    body = await request.json();
  } catch {
    return new NextResponse(null, { status: 400 });
  }

  const message = field(body.message, MAX.message);
  const name = oneLine(field(body.name, MAX.name));
  const contact = oneLine(field(body.contact, MAX.contact));
  if (!message) {
    return NextResponse.json(
      { error: "Please write some feedback." },
      { status: 400 },
    );
  }

  const ip = request.headers.get("x-real-ip") ?? "unknown";
  if (rateLimited(ip)) {
    return NextResponse.json(null, { status: 429 });
  }

  const { error } = await new Resend(process.env.RESEND_API_KEY).emails.send({
    from: "Escapade <onboarding@resend.dev>",
    to: process.env.FEEDBACK_TO as string,
    subject: name ? `Escapade feedback from ${name}` : "Escapade feedback",
    replyTo: looksLikeEmail(contact) ? contact : undefined,
    text: [
      message,
      "",
      "—",
      `Name: ${name || "(not given)"}`,
      `Contact: ${contact || "(not given)"}`,
    ].join("\n"),
  });

  if (error) {
    console.error("Feedback: Resend error", error);
    return NextResponse.json(null, { status: 500 });
  }

  return NextResponse.json({ ok: true });
}
