import type { BoardMark } from "../../constants";

export type BoardPieces = (number | null)[];

export type BoardMarks = Partial<Record<number, BoardMark[]>>;
