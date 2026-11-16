import { TRAY_MARK, type Side, type TrayMark } from "../../constants";
import PieceTooltip from "../piece/PieceTooltip";
import styles from "./Tray.module.css";

type TrayMarks = Partial<Record<number, TrayMark[]>>;

const MARKS = Object.values(TRAY_MARK);

export default function Tray({
  available,
  marks,
  live,
  actor,
  onPieceClick,
}: {
  available: boolean[];
  marks: TrayMarks;
  live: boolean;
  actor: Side | null;
  onPieceClick: (piece: number) => void;
}) {
  return (
    <div className={styles.tray} data-actor={actor}>
      {available.map((isAvailable, piece) => {
        const slotMarks = marks[piece] ?? [];

        return (
          <button
            key={piece}
            type="button"
            className={styles.slot}
            disabled={!live || !isAvailable}
            onClick={() => onPieceClick(piece)}
          >
            {MARKS.map((mark) => (
              <span
                key={mark}
                className={styles.mark}
                data-mark={mark}
                data-on={slotMarks.includes(mark)}
              />
            ))}

            <img
              src={`/pieces/piece${piece}.svg`}
              alt=""
              className={styles.piece}
              data-available={isAvailable}
              draggable={false}
            />
            {isAvailable && <PieceTooltip piece={piece} />}
          </button>
        );
      })}
    </div>
  );
}
