/** Visual annotations a square can carry. Several can stack on one square. */
export const ALL_MARKS = ["last-move"] as const;

export type BoardMark = (typeof ALL_MARKS)[number];

export type BoardPieces = (number | null)[];

export type BoardMarks = Partial<Record<number, BoardMark[]>>;
