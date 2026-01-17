/**
 * API Contract: Frontend ↔ Backend
 *
 * Philosophy: Frontend is "dumb" - it only displays what the backend tells it.
 * All game logic (win detection, move validation, etc.) is handled server-side.
 */

import type { AnalysisResult } from "./engine";

// ============================================================================
// REQUEST
// ============================================================================

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

// ============================================================================
// RESPONSE
// ============================================================================

/**
 * Quarto information
 */
export interface Quarto {
  intersection: [number, number, number, number];
  attribute: string;
}

/**
 * Move that the player can make
 */
export interface PlayerMove {
  piece?: number; // Piece to select (if selecting from tray)
  square?: number; // Square to place on (if placing piece)
  hex: string; // Internal game state representation
  description: string; // Human-readable description of this move
  quarto?: Quarto[]; // If this move creates a quarto, the winning line(s)
}

/**
 * Normal game loop - engine makes a move and gives piece to player
 */
export interface NormalMoveResponse {
  event: "normal";

  // Complete game state
  board: (number | undefined)[]; // 16 squares, undefined = empty
  tray: boolean[]; // 16 pieces, true = available

  // The piece the engine selected for the player to place
  pieceToPlace: number;

  // All legal moves the player can make
  moves: PlayerMove[];

  // Human-readable description of what just happened
  description: string;
  analysis: AnalysisResult
}

/**
 * Engine wins with a quarto
 */
export interface EngineWinResponse {
  event: "engine_win";

  // Final game state
  board: (number | undefined)[];
  tray: boolean[];

  // Squares to highlight (the winning line)
  highlighted: boolean[]; // 16 booleans

  // Human-readable description of the win
  description: string;
  analysis: AnalysisResult
}

/**
 * Game ends in a draw (board full, no quarto)
 */
export interface DrawResponse {
  event: "draw";

  // Final game state
  board: (number | undefined)[];
  tray: boolean[];

  // Human-readable description
  description: string;
  analysis: AnalysisResult
}

/**
 * All possible API responses
 */
export type EngineResponse =
  | NormalMoveResponse
  | EngineWinResponse
  | DrawResponse;

// ============================================================================
// FRONTEND RESPONSIBILITIES
// ============================================================================

/**
 * The frontend should:
 * 1. Display the board and tray as provided by the backend
 * 2. Allow the player to select from available moves
 * 3. Send the selected move's hex to the backend
 * 4. Display descriptions as provided
 * 5. Highlight squares as indicated by the backend
 *
 * The frontend should NOT:
 * 1. Check for winning conditions
 * 2. Validate moves
 * 3. Calculate available moves
 * 4. Determine piece placement logic
 */

// ============================================================================
// BACKEND RESPONSIBILITIES
// ============================================================================

/**
 * The backend should:
 * 1. Validate all moves
 * 2. Detect winning conditions (quartos)
 * 3. Calculate all legal moves for the player
 * 4. Generate human-readable descriptions
 * 5. Determine which squares to highlight
 * 6. Maintain complete game state
 * 7. Apply engine move selection based on strength
 */
