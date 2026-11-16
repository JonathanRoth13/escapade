import type { Side } from "../side";
import PieceTooltip from "../piece/PieceTooltip";
import styles from "./Tray.module.css";

/** Visual annotations a tray slot can carry. */
const ALL_MARKS = ["picked"] as const;

type TrayMark = (typeof ALL_MARKS)[number];

type TrayMarks = Partial<Record<number, TrayMark[]>>;

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
            {ALL_MARKS.map((mark) => (
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
