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
  actions: { label: string; onClick: () => void }[];
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
        {actions.map(({ label, onClick }) => (
          <button
            key={label}
            type="button"
            className={styles.button}
            onClick={onClick}
          >
            {label}
          </button>
        ))}
      </div>
    </div>
  );
}
