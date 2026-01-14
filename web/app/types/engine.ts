export enum Perspective {
  First = "first",
  Second = "second",
  Current = "current",
}

export interface Quarto {
  intersection: [number, number, number, number];
  attribute: string;
}

export interface Move {
  square?: number;
  piece?: number;
  hex: string;
  quarto?: Quarto[];
}

export interface Orbit {
  canon_hex: string;
  outcome: number;
  moves: Move[];
}

export interface AnalysisResult {
  ply_grid: string;
  ply_hex: string;
  canon_hex: string;
  quartos?: Quarto[];
  orbits?: Orbit[];
}

export type ApiResponse =
  | {
      event: "player moves first";
      board: (number | null)[];
      trayPieces: boolean[];
      moves: Move[];
    }
  | {
      event: "normal loop";
      engine_piece: number;
      board: (number | undefined)[];
      trayPieces: boolean[];
      moves: Move[];
      last_move_description?: string;
    }
  | {
      event: "engine quarto";
      board: (number | undefined)[];
      trayPieces: boolean[];
      boarder: boolean[];
      description: string;
      outcome?: "player_win" | "engine_win" | "draw";
    }
  | {
      event: "player quarto";
      board: (number | undefined)[];
      trayPieces: boolean[];
      boarder: boolean[];
      description: string;
    };
