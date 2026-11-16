import styles from "./CoachCard.module.css";

export default function CoachCard({
  title,
  actions,
}: {
  title: string;
  actions: { label: string; onClick: () => void }[];
}) {
  return (
    <div className={styles.card} role="dialog" aria-label={title}>
      <h2 className={styles.title}>{title}</h2>
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
