import type { ReactNode } from "react";
import styles from "./CoachCard.module.css";

export default function CoachCard({
  title,
  children,
  actions,
  delay = 0,
}: {
  title?: string;
  children?: ReactNode;
  actions: { label: string; onClick: () => void; secondary?: boolean }[];
  /** Milliseconds before the card fades in. */
  delay?: number;
}) {
  return (
    <div
      className={styles.card}
      role="dialog"
      aria-label={title}
      style={{ animationDelay: `${delay}ms` }}
    >
      {title && <h2 className={styles.title}>{title}</h2>}
      {children && <div className={styles.body}>{children}</div>}
      <div className={styles.actions}>
        {actions.map(({ label, onClick, secondary }) => (
          <button
            key={label}
            type="button"
            className={styles.button}
            data-secondary={secondary}
            onClick={onClick}
          >
            {label}
          </button>
        ))}
      </div>
    </div>
  );
}
