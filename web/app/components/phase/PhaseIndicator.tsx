import type { CSSProperties } from "react";
import { PHASE, PHASE_LABEL, PHASE_SIDE, type Phase } from "../../constants";
import styles from "./PhaseIndicator.module.css";

const PHASES = Object.values(PHASE);

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
          {PHASE_LABEL[step]}
        </li>
      ))}
    </ol>
  );
}
