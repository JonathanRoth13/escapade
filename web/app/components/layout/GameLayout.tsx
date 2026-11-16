import type { ReactNode } from "react";
import styles from "./GameLayout.module.css";

export default function GameLayout({
  tray,
  board,
  opponent,
  phase,
  you,
  headerActions,
}: {
  tray: ReactNode;
  board: ReactNode;
  opponent: ReactNode;
  phase: ReactNode;
  you: ReactNode;
  headerActions: ReactNode;
}) {
  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <span className={styles.title}>ESCAPADE</span>
        <div className={styles.headerActions}>{headerActions}</div>
      </header>
      <main className={styles.main}>
        <div className={styles.layout}>
          <div className={styles.tray}>{tray}</div>
          <div className={styles.board}>{board}</div>
          <div className={styles.opponent}>{opponent}</div>
          <div className={`${styles.phase} ${styles.panel}`}>{phase}</div>
          <div className={styles.you}>{you}</div>
        </div>
        <p className={styles.unsupported}>
          Escapade isn&apos;t supported on this screen size yet.
        </p>
      </main>
    </div>
  );
}
