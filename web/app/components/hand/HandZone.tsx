import type { Side } from "../../constants";
import PieceTooltip from "../piece/PieceTooltip";
import styles from "./HandZone.module.css";

export default function HandZone({
  side,
  label,
  piece,
  active,
}: {
  side: Side;
  label: string;
  piece: number | null;
  active: boolean;
}) {
  return (
    <div className={styles.zone} data-side={side} data-active={active}>
      <span className={styles.label}>{label}</span>
      <div className={styles.slot}>
        {piece !== null && (
          <>
            <img
              key={piece}
              src={`/pieces/piece${piece}.svg`}
              alt=""
              className={styles.piece}
              draggable={false}
            />
            <PieceTooltip piece={piece} />
          </>
        )}
      </div>
    </div>
  );
}
