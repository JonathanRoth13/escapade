export const SIDE = {
  YOU: "you",
  OPPONENT: "opponent",
} as const;

export type Side = (typeof SIDE)[keyof typeof SIDE];

export const PHASE = {
  YOU_SELECT: "you-select",
  OPPONENT_PLACES: "opponent-places",
  OPPONENT_SELECTS: "opponent-selects",
  YOU_PLACE: "you-place",
} as const;

export type Phase = (typeof PHASE)[keyof typeof PHASE];

export const PHASE_SIDE: Record<Phase, Side> = {
  [PHASE.YOU_SELECT]: SIDE.YOU,
  [PHASE.OPPONENT_PLACES]: SIDE.OPPONENT,
  [PHASE.OPPONENT_SELECTS]: SIDE.OPPONENT,
  [PHASE.YOU_PLACE]: SIDE.YOU,
};

export const BOARD_MARK = {
  LAST_MOVE: "last-move",
} as const;

export type BoardMark = (typeof BOARD_MARK)[keyof typeof BOARD_MARK];

export const TRAY_MARK = {
  PICKED: "picked",
} as const;

export type TrayMark = (typeof TRAY_MARK)[keyof typeof TRAY_MARK];
