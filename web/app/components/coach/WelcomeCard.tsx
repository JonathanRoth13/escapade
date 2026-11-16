import { SIDE, type Side } from "../../constants";
import { RULES_URL } from "../../links";
import CoachCard from "./CoachCard";

export default function WelcomeCard({
  onStart,
}: {
  onStart: (first: Side) => void;
}) {
  return (
    <CoachCard
      actions={[
        { label: "Go first", onClick: () => onStart(SIDE.YOU) },
        { label: "Go second", onClick: () => onStart(SIDE.OPPONENT) },
      ]}
    >
      <p>
        Hi, I&apos;m Jonathan Roth. I&apos;m a full-stack software engineer, and
        Escapade is my portfolio piece.
      </p>
      <p>
        Please enjoy playing Quarto against an engine. If you are new to the
        game,{" "}
        <a href={RULES_URL} target="_blank" rel="noopener noreferrer">
          the rules
        </a>{" "}
        take a minute to learn.
      </p>
    </CoachCard>
  );
}
