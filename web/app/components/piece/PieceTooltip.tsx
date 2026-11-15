import { pieceAttributes } from "./attributes";
import styles from "./PieceTooltip.module.css";

export default function PieceTooltip({ piece }: { piece: number }) {
  return (
    <span className={styles.tooltip} role="tooltip">
      {pieceAttributes(piece).join(" · ")}
    </span>
  );
}
