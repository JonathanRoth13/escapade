import type { Move, AnalysisResult } from "./engine";
import type { PlayerMove } from "./api";

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
  isRightPanelTray: boolean;
  isRightPanelLocked: boolean;

  // Play mode state
  pieceToPlace?: number;
  //pieceToPlace?: number;
  moves?: PlayerMove[];
  movesTerminal?: Move[];

  square?: number;

  displayPlayAgain: boolean;

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
  isRightPanelTray: true,
  isRightPanelLocked: true,
  displayPlayAgain: false,
  engineStrength: 100,
};

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
      boarder: boolean[];
      description: string;
      outcome: "player_win" | "engine_win" | "draw";
      analysis?: AnalysisResult;
    }
  | {
      type: "PLAYER_QUARTO";
      board: (number | undefined)[];
      tray: boolean[];
      boarder: boolean[];
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
        isRightPanelTray: true,
        isRightPanelLocked: false,
        isBoardLocked: true,
        pieceToPlace: undefined,
        square: undefined,
        engineStrength: state.engineStrength,
        moves: [
          { piece: 0, hex: "0000000000000000000000", description: "You selected piece 0." },
          { piece: 1, hex: "0000000000000000000010", description: "You selected piece 1." },
          { piece: 2, hex: "0000000000000000000020", description: "You selected piece 2." },
          { piece: 3, hex: "0000000000000000000030", description: "You selected piece 3." },
          { piece: 4, hex: "0000000000000000000040", description: "You selected piece 4." },
          { piece: 5, hex: "0000000000000000000050", description: "You selected piece 5." },
          { piece: 6, hex: "0000000000000000000060", description: "You selected piece 6." },
          { piece: 7, hex: "0000000000000000000070", description: "You selected piece 7." },
          { piece: 8, hex: "0000000000000000000080", description: "You selected piece 8." },
          { piece: 9, hex: "0000000000000000000090", description: "You selected piece 9." },
          { piece: 10, hex: "00000000000000000000A0", description: "You selected piece 10." },
          { piece: 11, hex: "00000000000000000000B0", description: "You selected piece 11." },
          { piece: 12, hex: "00000000000000000000C0", description: "You selected piece 12." },
          { piece: 13, hex: "00000000000000000000D0", description: "You selected piece 13." },
          { piece: 14, hex: "00000000000000000000E0", description: "You selected piece 14." },
          { piece: 15, hex: "00000000000000000000F0", description: "You selected piece 15." },
        ],
        displayPlayAgain: false,
        analysisData: {
          ply_grid: "                 ",
          ply_hex: "FFFFFFFFFFFFFFFFFFFFF0",
          canon_hex: "FFFFFFFFFFFFFFFFFFFFF0",
          orbits: [
            {
              canon_hex: "0000000000000000000000",
              outcome: 15,
              moves: [
                { piece: 0, hex: "0000000000000000000000" },
                { piece: 1, hex: "0000000000000000000010" },
                { piece: 2, hex: "0000000000000000000020" },
                { piece: 3, hex: "0000000000000000000030" },
                { piece: 4, hex: "0000000000000000000040" },
                { piece: 5, hex: "0000000000000000000050" },
                { piece: 6, hex: "0000000000000000000060" },
                { piece: 7, hex: "0000000000000000000070" },
                { piece: 8, hex: "0000000000000000000080" },
                { piece: 9, hex: "0000000000000000000090" },
                { piece: 10, hex: "00000000000000000000A0" },
                { piece: 11, hex: "00000000000000000000B0" },
                { piece: 12, hex: "00000000000000000000C0" },
                { piece: 13, hex: "00000000000000000000D0" },
                { piece: 14, hex: "00000000000000000000E0" },
                { piece: 15, hex: "00000000000000000000F0" },
              ],
            },
          ],
        },
      };

    case "NORMAL_MOVE":
      const trayNormal: TrayPiece[] = action.tray.map((available: boolean) => ({
        available,
        border: false,
      }));
      const boardNormal: Cell[] = action.board.map((piece) => ({
        piece,
        highlighted: false,
      }));
      return {
        mode: state.mode,
        board: boardNormal,
        tray: trayNormal,
        moveHistory: state.moveHistory,
        isRightPanelTray: false,
        pieceToPlace: action.pieceToPlace,
        isRightPanelLocked: true,
        isBoardLocked: false,
        engineStrength: state.engineStrength,
        moves: action.moves,
        displayPlayAgain: false,
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
          isRightPanelTray: true,
          pieceToPlace: undefined,
          isRightPanelLocked: true,
          isBoardLocked: true,
          engineStrength: state.engineStrength,
          moves: undefined,
          outcome: "player_win",
          analysisData: state.analysisData,
          displayPlayAgain: false,
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
          isRightPanelTray: true,
          pieceToPlace: undefined,
          isRightPanelLocked: true,
          isBoardLocked: true,
          engineStrength: state.engineStrength,
          moves: undefined,
          outcome: "draw",
          analysisData: state.analysisData,
          displayPlayAgain: false,
        };
      }

      // Normal move - game continues
      return {
        ...state,
        board: newBoard,
        tray: newTray,
        isRightPanelTray: true,
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
      const trayIIII: TrayPiece[] = action.tray.map((available: boolean) => ({
        available,
        border: false,
      }));
      const newBoardIIII = Array(16);
      for (let i = 0; i < 16; i++) {
        newBoardIIII[i] = {
          piece: action.board[i],
          highlighted: action.boarder[i],
        };
      }
      console.log(action.description);
      return {
        mode: state.mode,
        board: newBoardIIII,
        tray: trayIIII,
        moveHistory: state.moveHistory,
        isRightPanelTray: true,
        pieceToPlace: undefined,
        isRightPanelLocked: true,
        isBoardLocked: true,
        engineStrength: state.engineStrength,
        moves: undefined,
        outcome: action.outcome,
        analysisData: action.analysis,
        displayPlayAgain: false,
      };



    case "DRAW":
      const trayDraw: TrayPiece[] = action.tray.map((available: boolean) => ({
        available,
        border: false,
      }));
      const boardDraw: Cell[] = action.board.map((piece) => ({
        piece,
        highlighted: false,
      }));
      console.log(action.description);
      return {
        mode: state.mode,
        board: boardDraw,
        tray: trayDraw,
        moveHistory: state.moveHistory,
        isRightPanelTray: true,
        pieceToPlace: undefined,
        isRightPanelLocked: true,
        isBoardLocked: true,
        engineStrength: state.engineStrength,
        moves: undefined,
        outcome: "draw",
        analysisData: action.analysis,
        displayPlayAgain: false,
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
