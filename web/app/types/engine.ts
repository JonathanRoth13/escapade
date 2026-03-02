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

