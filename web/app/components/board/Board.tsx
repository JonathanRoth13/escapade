import PieceTooltip from "../piece/PieceTooltip";
import styles from "./Board.module.css";
import { BOARD_MARK, type Side } from "../../constants";
import type { BoardMarks, BoardPieces } from "./types";

const DISPLAY_ORDER = [12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3];

const MARKS = Object.values(BOARD_MARK);

/**
 * Layered board, modelled on lichess's chessground:
 *   1. grid    – lines drawn once by the container (gap over a line colour),
 *                so every line has the same weight and meets cleanly
 *   2. marks   – per-square tint layers; always rendered and faded with
 *                opacity so they transition both in and out, plus a hover
 *                dot on empty squares (every empty square is playable in
 *                Quarto, so this needs no input from the caller)
 *   3. pieces  – fade in when they first appear
 * Highlights never use borders, so they never change the board's geometry.
 *
 * `live` turns on input: empty squares take clicks and show the hover dot.
 * Emphasis is separate: `actor` rings the board in the colour of whoever
 * acts here.
 *
 * The board fills its container's width and stays square; the layout decides
 * its size.
 */
export default function Board({
  pieces,
  marks,
  live,
  actor,
  onSquareClick,
}: {
  pieces: BoardPieces;
  marks: BoardMarks;
  live: boolean;
  actor: Side | null;
  onSquareClick: (square: number) => void;
}) {
  return (
    <div className={styles.board} data-actor={actor}>
      {DISPLAY_ORDER.map((square) => {
        const piece = pieces[square];
        const squareMarks = marks[square] ?? [];
        const empty = piece === null;

        return (
          <button
            key={square}
            type="button"
            className={styles.square}
            disabled={!live || !empty}
            onClick={() => onSquareClick(square)}
          >
            {MARKS.map((mark) => (
              <span
                key={mark}
                className={styles.mark}
                data-mark={mark}
                data-on={squareMarks.includes(mark)}
              />
            ))}
            <span className={styles.hover} />

            {!empty && (
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
          </button>
        );
      })}
    </div>
  );
}
