import { SIDE, type Side } from "../../constants";
import type { WinLine } from "../../game/useGame";
import { ATTRIBUTE_NAMES } from "../piece/attributes";
import CoachCard from "./CoachCard";

/** Milliseconds before the card fades in, so the winning line is seen first. */
const DELAY = 1200;

const RESULT: Record<Side, string> = {
  [SIDE.YOU]: "Quarto! You win",
  [SIDE.OPPONENT]: "Quarto! Your opponent wins",
};

/** "tall and dark", "tall, dark and round", … */
function listWords(words: string[]): string {
  return words.length < 2
    ? (words[0] ?? "")
    : `${words.slice(0, -1).join(", ")} and ${words.at(-1)}`;
}

/** What the winning line(s) share, one clause per line (a square is on at most three). */
function describe(lines: WinLine[]): string {
  const shared = lines.map((line) =>
    listWords(line.attributes.map((id) => ATTRIBUTE_NAMES[id])),
  );
  switch (shared.length) {
    case 1:
      return `All four pieces in the line are ${shared[0]}.`;
    case 2:
      return `Two lines at once. One is all ${shared[0]}; the other is all ${shared[1]}.`;
    default:
      return `Three lines at once. One is all ${shared[0]}; another is all ${shared[1]}; the third is all ${shared[2]}.`;
  }
}

export default function GameOverCard({
  winner,
  lines,
  onPlayAgain,
}: {
  winner: Side | null;
  lines: WinLine[];
  onPlayAgain: (first: Side) => void;
}) {
  return (
    <CoachCard
      title={winner ? RESULT[winner] : "It's a draw"}
      delay={DELAY}
      actions={[
        { label: "Go first", onClick: () => onPlayAgain(SIDE.YOU) },
        { label: "Go second", onClick: () => onPlayAgain(SIDE.OPPONENT) },
      ]}
    >
      <p>
        {winner
          ? describe(lines)
          : "The board is full and no line shares an attribute."}
      </p>
      <p>Play again?</p>
    </CoachCard>
  );
}
