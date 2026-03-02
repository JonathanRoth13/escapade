import type { AnalysisResult } from "./engine";
import type { PlayerMove } from "./api";
import { PLAYER_MOVES_FIRST_DATA } from "../constants/gameStart";

// Core game types
export type Cell = {
  piece?: number;
  highlighted: boolean;
};

export type Board = Cell[];

export type TrayPiece = {
  available: boolean;
  border: boolean;
};

export type Mode = "landing" | "play";

export interface GameState {
  mode: Mode;

  board: Board;
  isBoardLocked: boolean;

  tray: TrayPiece[];
  isRightPanelLocked: boolean;

  // Play mode state
  pieceToPlace?: number;
  moves?: PlayerMove[];

  square?: number;

  outcome?: "player_win" | "engine_win" | "draw";
  analysisData?: AnalysisResult;
  moveHistory: string[];

  engineStrength: number;
}

export const initialGameState: GameState = {
  mode: "landing",
  board: Array.from({ length: 16 }, () => ({
    piece: undefined,
    highlighted: false,
  })),
  isBoardLocked: true,
  tray: Array.from({ length: 16 }, () => ({
    available: true,
    border: false,
  })),
  moveHistory: [],
  isRightPanelLocked: true,
  engineStrength: 100,
};

function trayFromBooleans(booleans: boolean[]): TrayPiece[] {
  return booleans.map((available) => ({ available, border: false }));
}

export type GameAction =
  // Session actions
  | { type: "RESET" }
  | { type: "START_PLAY_MODE" }

  // Play mode actions
  | { type: "PLAYER_MOVE_FIRST" }
  | { type: "SELECT_PIECE"; pieceId: number }
  | { type: "PLACE_PIECE"; square: number }
  | {
      type: "NORMAL_MOVE";
      moves: PlayerMove[];
      board: (number | undefined)[];
      tray: boolean[];
      pieceToPlace: number;
      description?: string;
      analysis?: AnalysisResult;
    }
  | {
      type: "ENGINE_QUARTO";
      board: (number | undefined)[];
      tray: boolean[];
      highlighted: boolean[];
      description: string;
      outcome: "player_win" | "engine_win" | "draw";
      analysis?: AnalysisResult;
    }
  | {
      type: "PLAYER_QUARTO";
      board: (number | undefined)[];
      tray: boolean[];
      highlighted: boolean[];
      description: string;
      analysis?: AnalysisResult;
    }
  | {
      type: "DRAW";
      board: (number | undefined)[];
      tray: boolean[];
      description: string;
      analysis?: AnalysisResult;
    }

  // Move history actions
  | { type: "ADD_MOVE_TO_HISTORY"; move: string }

  // Shared actions
  | { type: "SET_STRENGTH"; strength: number };

export function gameReducer(state: GameState, action: GameAction): GameState {
  switch (action.type) {
    case "RESET":
      return { ...initialGameState, engineStrength: state.engineStrength }; // this should be a hard reset of everything except engine strength

    case "START_PLAY_MODE":
      return {
        ...initialGameState,
        engineStrength: state.engineStrength,
        mode: "play",
      };

    case "PLAYER_MOVE_FIRST":
      return {
        mode: state.mode,
        board: Array.from({ length: 16 }, () => ({
          piece: undefined,
          highlighted: false,
        })),
        tray: Array.from({ length: 16 }, () => ({
          available: true,
          border: false,
        })),
        moveHistory: state.moveHistory,
        isRightPanelLocked: false,
        isBoardLocked: true,
        pieceToPlace: undefined,
        square: undefined,
        engineStrength: state.engineStrength,
        moves: PLAYER_MOVES_FIRST_DATA.moves,
        analysisData: PLAYER_MOVES_FIRST_DATA.analysis,
      };

    case "NORMAL_MOVE":
      return {
        mode: state.mode,
        board: action.board.map((piece) => ({ piece, highlighted: false })),
        tray: trayFromBooleans(action.tray),
        moveHistory: state.moveHistory,
        pieceToPlace: action.pieceToPlace,
        isRightPanelLocked: true,
        isBoardLocked: false,
        engineStrength: state.engineStrength,
        moves: action.moves,
        analysisData: action.analysis,
      };

    case "PLACE_PIECE":
      // Update the board at the given square with the piece in hand
      const newBoard = state.board.map((cell, index) => {
        if (index === action.square) {
          return {
            piece: state.pieceToPlace,
            highlighted: false,
          };
        }
        return cell;
      });

      // Update tray to mark the placed piece as unavailable
      const newTray = state.tray.map((trayPiece, index) => {
        if (index === state.pieceToPlace) {
          return {
            ...trayPiece,
            available: false,
          };
        }
        return trayPiece;
      });

      // Check if this move is a quarto
      const quartoMove = state.moves?.find(
        (move) => move.square === action.square && move.quarto,
      );

      if (quartoMove) {
        // Player wins with quarto
        const highlighted = new Array(16).fill(false);
        const squares = quartoMove.quarto!.flatMap((q) => q.intersection);
        for (let i = 0; i < 16; i++) {
          highlighted[i] = squares.includes(i);
        }

        const boardWithHighlights = newBoard.map((cell, index) => ({
          ...cell,
          highlighted: highlighted[index],
        }));

        return {
          mode: state.mode,
          board: boardWithHighlights,
          tray: newTray,
          moveHistory: [...state.moveHistory, quartoMove.description],
          pieceToPlace: undefined,
          isRightPanelLocked: true,
          isBoardLocked: true,
          engineStrength: state.engineStrength,
          moves: undefined,
          outcome: "player_win",
          analysisData: state.analysisData,
        };
      }

      // Check if this move is a draw (has square but no piece and no quarto)
      const drawMove = state.moves?.find(
        (move) =>
          move.square === action.square &&
          !move.quarto &&
          move.piece === undefined,
      );

      if (drawMove) {
        // Draw - player placed last piece
        return {
          mode: state.mode,
          board: newBoard,
          tray: newTray,
          moveHistory: [...state.moveHistory, drawMove.description],
          pieceToPlace: undefined,
          isRightPanelLocked: true,
          isBoardLocked: true,
          engineStrength: state.engineStrength,
          moves: undefined,
          outcome: "draw",
          analysisData: state.analysisData,
        };
      }

      // Normal move - game continues
      return {
        ...state,
        board: newBoard,
        tray: newTray,
        isRightPanelLocked: false,
        pieceToPlace: undefined,
        isBoardLocked: true,
        square: action.square,
      };

    case "SET_STRENGTH":
      return {
        ...state,
        engineStrength: action.strength,
      };

    case "ENGINE_QUARTO":
      const engineQuartoBoard: Cell[] = action.board.map((piece, i) => ({
        piece,
        highlighted: action.highlighted[i],
      }));
      return {
        mode: state.mode,
        board: engineQuartoBoard,
        tray: trayFromBooleans(action.tray),
        moveHistory: state.moveHistory,
        pieceToPlace: undefined,
        isRightPanelLocked: true,
        isBoardLocked: true,
        engineStrength: state.engineStrength,
        moves: undefined,
        outcome: action.outcome,
        analysisData: action.analysis,
      };

    case "DRAW":
      return {
        mode: state.mode,
        board: action.board.map((piece) => ({ piece, highlighted: false })),
        tray: trayFromBooleans(action.tray),
        moveHistory: state.moveHistory,
        pieceToPlace: undefined,
        isRightPanelLocked: true,
        isBoardLocked: true,
        engineStrength: state.engineStrength,
        moves: undefined,
        outcome: "draw",
        analysisData: action.analysis,
      };

    case "ADD_MOVE_TO_HISTORY":
      return {
        ...state,
        moveHistory: [...state.moveHistory, action.move],
      };

    default:
      return state;
  }
}
