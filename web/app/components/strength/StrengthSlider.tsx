import styles from "./StrengthSlider.module.css";

export default function StrengthSlider({
  value,
  onChange,
}: {
  value: number;
  onChange: (value: number) => void;
}) {
  return (
    <div className={styles.slider}>
      <label className={styles.label}>
        <span>Opponent strength</span>
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
        <span>Random</span>
        <span>Perfect</span>
      </div>
    </div>
  );
}
