import styles from "./StrengthSlider.module.css";

export default function StrengthSlider({
  value,
  onChange,
  highlighted,
}: {
  value: number;
  onChange: (value: number) => void;
  highlighted: boolean;
}) {
  return (
    <div className={styles.slider} data-highlighted={highlighted}>
      <label className={styles.label}>
        <span className={styles.title}>Difficulty</span>
        <span className={styles.value}>{value}</span>
        <input
          type="range"
          min={0}
          max={100}
          value={value}
          onChange={(e) => onChange(Number(e.target.value))}
          className={styles.input}
        />
      </label>
      <div className={styles.ends}>
        <span>Easy</span>
        <span>Less Easy</span>
      </div>
    </div>
  );
}
