import type { ReactNode } from "react";
import styles from "./Dialog.module.css";

/** A header button that opens a card over the page. Clicking outside the card, or ×, closes it. */
export default function Dialog({
  label,
  title,
  open,
  onOpen,
  onClose,
  children,
}: {
  label: string;
  title: string;
  open: boolean;
  onOpen: () => void;
  onClose: () => void;
  children: ReactNode;
}) {
  return (
    <>
      <button type="button" className={styles.trigger} onClick={onOpen}>
        {label}
      </button>

      {open && (
        <div className={styles.backdrop} onClick={onClose}>
          <div
            className={styles.card}
            role="dialog"
            aria-label={title}
            onClick={(e) => e.stopPropagation()}
          >
            <div className={styles.header}>
              <h2 className={styles.title}>{title}</h2>
              <button
                type="button"
                className={styles.close}
                aria-label="Close"
                onClick={onClose}
              >
                ×
              </button>
            </div>
            {children}
          </div>
        </div>
      )}
    </>
  );
}
