import type { NormalMoveResponse } from "../types/api";

// Empty board ply string for starting a new game (player moves second)
export const EMPTY_BOARD_PLY = "                 "; // 17 spaces: 16 for board + 1 for piece

// Initial game state when player moves first
export const PLAYER_MOVES_FIRST_DATA: NormalMoveResponse = {
  event: "normal",
  board: Array.from({ length: 16 }, () => undefined),
  tray: Array.from({ length: 16 }, () => true),
  pieceToPlace: 0,
  description: "Select a piece for your opponent to place.",
  analysis: {
    ply_grid: "                 ",
    ply_hex: "FFFFFFFFFFFFFFFFFFFFF0",
    canon_hex: "FFFFFFFFFFFFFFFFFFFFF0",
    orbits: [
      {
        canon_hex: "0000000000000000000000",
        outcome: 15,
        moves: Array.from({ length: 16 }, (_, i) => ({
          piece: i,
          hex: `00000000000000000000${i.toString(16).toUpperCase()}0`,
        })),
      },
    ],
  },
  moves: Array.from({ length: 16 }, (_, i) => ({
    piece: i,
    hex: `00000000000000000000${i.toString(16).toUpperCase()}0`,
    description: `You selected piece ${i}.`,
  })),
};
