import { PHASE, SIDE, type Phase, type Side } from "../constants";

/** One step of the game, not a ply */
export type Move =
  | { kind: "select"; by: Side; piece: number }
  | { kind: "place"; by: Side; piece: number; square: number };

export type Position = {
  board: (number | null)[];
  available: boolean[];
  hands: Record<Side, number | null>;
  lastMove: number | null;
};

const START: Position = {
  board: Array(16).fill(null),
  available: Array(16).fill(true),
  hands: { [SIDE.YOU]: null, [SIDE.OPPONENT]: null },
  lastMove: null,
};

const otherSide = (side: Side): Side =>
  side === SIDE.YOU ? SIDE.OPPONENT : SIDE.YOU;

function applyMove(position: Position, move: Move): Position {
  const { board, available, hands } = position;
  switch (move.kind) {
    case "select":
      return {
        ...position,
        available: available.with(move.piece, false),
        hands: { ...hands, [otherSide(move.by)]: move.piece },
      };
    case "place":
      return {
        ...position,
        board: board.with(move.square, move.piece),
        hands: { ...hands, [move.by]: null },
        lastMove: move.square,
      };
  }
}

export function positionAfter(moves: Move[]): Position {
  let position = START;
  for (const move of moves) position = applyMove(position, move);
  return position;
}

const SELECT_PHASE: Record<Side, Phase> = {
  [SIDE.YOU]: PHASE.YOU_SELECT,
  [SIDE.OPPONENT]: PHASE.OPPONENT_SELECTS,
};

const PLACE_PHASE: Record<Side, Phase> = {
  [SIDE.YOU]: PHASE.YOU_PLACE,
  [SIDE.OPPONENT]: PHASE.OPPONENT_PLACES,
};

export const openingPhase = (first: Side): Phase => SELECT_PHASE[first];

export function nextPhase(move: Move): Phase {
  if (move.kind === "select") return PLACE_PHASE[otherSide(move.by)];
  return SELECT_PHASE[move.by];
}
