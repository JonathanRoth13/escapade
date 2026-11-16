/**
 * API Contract: Frontend ↔ Backend
 *
 * Philosophy: Frontend is "dumb" - it only displays what the backend tells it.
 * All game logic (win detection, move validation, etc.) is handled server-side.
 */

import type { AnalysisResult } from "./engine";

/**
 * GET /api/play
 *
 * Query Parameters:
 * - ply: string - The current game position in ply format (17 chars: 16 board + 1 piece)
 * - strength: number (0-100) - Engine strength (100 = perfect play, 0 = random)
 */
export interface EngineRequest {
  ply: string;
  strength: number;
}

export interface Quarto {
  intersection: [number, number, number, number];
  attribute: number; // id 0-7, see ATTRIBUTE_NAMES
}

export interface PlayerMove {
  piece?: number; // Piece to select (if selecting from tray)
  square?: number; // Square to place on (if placing piece)
  hex: string; // Internal game state representation
  description: string;
  quarto?: Quarto[]; // If this move creates a quarto, the winning line(s)
}

/**
 * Normal game loop - engine makes a move and gives piece to player
 */
export interface NormalMoveResponse {
  event: "normal";
  board: (number | undefined)[]; // 16 squares, undefined = empty
  tray: boolean[]; // 16 pieces, true = available
  pieceToPlace: number;
  moves: PlayerMove[];
  description: string;
  analysis: AnalysisResult
}

export interface EngineWinResponse {
  event: "engine_win";
  board: (number | undefined)[];
  tray: boolean[];
  highlighted: boolean[]; // 16 squares, true = on a winning line
  quartos: Quarto[];
  description: string;
  analysis: AnalysisResult
}

export interface DrawResponse {
  event: "draw";
  board: (number | undefined)[];
  tray: boolean[];
  description: string;
  analysis: AnalysisResult
}

export type EngineResponse =
  | NormalMoveResponse
  | EngineWinResponse
  | DrawResponse;
