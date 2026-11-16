import type { CSSProperties } from "react";
import { PHASE, PHASE_SIDE, type Phase } from "../../constants";
import styles from "./PhaseIndicator.module.css";

const PHASES = Object.values(PHASE);

const LABEL: Record<Phase, string> = {
  [PHASE.YOU_SELECT]: "Player selects a piece",
  [PHASE.OPPONENT_PLACES]: "Opponent places the piece",
  [PHASE.OPPONENT_SELECTS]: "Opponent selects a piece",
  [PHASE.YOU_PLACE]: "Player places the piece",
};

/**
 * The four-step turn cycle with the current step lit. `phase: null`
 * (e.g. game over) lights nothing.
 *
 * On desktop the highlight is one pill that slides between steps and takes
 * on the current side's colour, so a phase change reads as movement.
 */
export default function PhaseIndicator({ phase }: { phase: Phase | null }) {
  const index = phase === null ? -1 : PHASES.indexOf(phase);

  return (
    <ol className={styles.steps}>
      <li
        className={styles.pill}
        data-side={phase && PHASE_SIDE[phase]}
        data-visible={phase !== null}
        style={{ "--index": Math.max(index, 0) } as CSSProperties}
        aria-hidden
      />
      {PHASES.map((step) => (
        <li
          key={step}
          className={styles.step}
          data-side={PHASE_SIDE[step]}
          data-current={step === phase}
        >
          <span className={styles.dot} />
          <span className={styles.label}>{LABEL[step]}</span>
        </li>
      ))}
    </ol>
  );
}
