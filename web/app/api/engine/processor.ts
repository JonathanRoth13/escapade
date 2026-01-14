import { AnalysisResult, Orbit, Move } from "@/app/types/engine";
import type { EngineResponse, PlayerMove } from "@/app/types/api";

function getBoard(ply_grid: string): (number | undefined)[] {
  const board: (number | undefined)[] = new Array(16);

  for (let i = 0; i < 16; i++) {
    const char = ply_grid[i];
    board[i] = char === " " ? undefined : parseInt(char, 16);
  }

  return board;
}

function getTray(ply_grid: string): boolean[] {
  const tray = new Array(16).fill(true);

  for (const char of ply_grid) {
    if (char !== " ") {
      tray[parseInt(char, 16)] = false;
    }
  }

  return tray;
}

function getLayer(ply_grid: string): number {
  return ply_grid.replace(/\s/g, "").length;
}

function getMovesFromOrbits(orbits: Orbit[]) {
  if (orbits.length === 0) {
    return { bestMoves: [], allMoves: [] };
  }

  const bestOutcome = orbits[0].outcome;
  const allMoves = orbits.flatMap((orbit) => orbit.moves);
  const bestMoves = orbits
    .filter((orbit) => orbit.outcome === bestOutcome)
    .flatMap((orbit) => orbit.moves);

  return { bestMoves, allMoves };
}

function selectMove(
  bestMoves: Move[],
  allMoves: Move[],
  strength: number,
): Move {
  // Quadratic curve: makes high values (like 90) more random than linear
  // Formula: randomProbability = 100 * (1 - (strength/100)^2)
  // Examples: 100→0%, 90→19%, 80→36%, 50→75%, 0→100%
  const normalizedStrength = strength / 100;
  const randomProbability = 100 * (1 - normalizedStrength * normalizedStrength);
  const roll = Math.random() * 100;

  if (roll < randomProbability) {
    return allMoves[Math.floor(Math.random() * allMoves.length)];
  }
  return bestMoves[Math.floor(Math.random() * bestMoves.length)];
}

function updateTrayAfterMove(
  tray: boolean[],
  pieceId: number | undefined,
): boolean[] {
  if (pieceId === undefined) return tray;
  const updated = [...tray];
  updated[pieceId] = false;
  return updated;
}

function extractMovesFromAnalysis(analysis: AnalysisResult): PlayerMove[] {
  const layer = getLayer(analysis.ply_grid);
  const pieceInHand = layer > 0 ? parseInt(analysis.ply_grid[16], 16) : undefined;

  return (
    analysis.orbits?.flatMap((orbit) =>
      orbit.moves.map((move) => {
        let description: string;

        if (move.quarto) {
          // This move creates a quarto
          const sentences = move.quarto.map(
            (q) =>
              `The pieces on squares ${q.intersection[0]}, ${q.intersection[1]}, ${q.intersection[2]}, and ${q.intersection[3]} are all ${q.attribute}.`,
          );
          description = `You placed piece ${pieceInHand} on square ${move.square}. Quarto! ${sentences.join(" ")}`;
        } else if (move.square !== undefined && move.piece === undefined) {
          // Last move - placing piece but no next piece (draw)
          description = `You placed piece ${pieceInHand} on square ${move.square}. The board is full. The game ends in a draw.`;
        } else if (move.square === undefined) {
          // Selecting a piece from tray (first move of game)
          description = `You selected piece ${move.piece}.`;
        } else if (pieceInHand !== undefined) {
          // Placing piece on board and selecting next piece
          description = `You placed piece ${pieceInHand} on square ${move.square} and selected piece ${move.piece}.`;
        } else {
          // Should not happen, but handle gracefully
          description = `You selected piece ${move.piece}.`;
        }

        return {
          piece: move.piece,
          hex: move.hex,
          square: move.square,
          description,
          quarto: move.quarto,
        };
      }),
    ) || []
  );
}

function buildQuartoDescription(
  placedPieceId: number,
  placedSquare: number,
  move: Move,
): string {
  if (!move.quarto) return "";

  const sentences = move.quarto.map(
    (q) =>
      `The pieces on squares ${q.intersection[0]}, ${q.intersection[1]}, ${q.intersection[2]}, and ${q.intersection[3]} are all ${q.attribute}.`,
  );

  return `The opponent placed piece ${placedPieceId} on square ${placedSquare}. Quarto! ${sentences.join(" ")}`;
}

function getHighlightedSquares(move: Move): boolean[] {
  const highlighted = new Array(16).fill(false);
  if (!move.quarto) return highlighted;

  const squares = move.quarto.flatMap((q) => q.intersection);
  for (let i = 0; i < 16; i++) {
    highlighted[i] = squares.includes(i);
  }

  return highlighted;
}

export async function processAnalysis(
  strength: number,
  analysis: AnalysisResult,
  analyzePosition: (ply: string) => Promise<AnalysisResult>,
): Promise<EngineResponse> {
  const tray = getTray(analysis.ply_grid);
  const layer = getLayer(analysis.ply_grid);

  // Player wins and draws are now handled client-side
  // Backend only handles engine moves

  const { bestMoves, allMoves } = getMovesFromOrbits(
    analysis.orbits as Orbit[],
  );
  const move = selectMove(bestMoves, allMoves, strength);

  // Handle Quarto (engine wins)
  if (move.quarto) {
    const board = getBoard(analysis.ply_grid);
    const placedPieceId = parseInt(analysis.ply_grid[16], 16);
    const placedSquare = move.square as number;
    board[placedSquare] = placedPieceId;

    return {
      event: "engine_win",
      board,
      tray: updateTrayAfterMove(tray, move.piece),
      highlighted: getHighlightedSquares(move),
      description: buildQuartoDescription(placedPieceId, placedSquare, move),
      analysis: {} as AnalysisResult,
    };
  }

  // Check for draw (engine placed last piece, no quarto)
  if (move.square !== undefined && move.piece === undefined) {
    const board = getBoard(analysis.ply_grid);
    const placedPieceId = parseInt(analysis.ply_grid[16], 16);
    const placedSquare = move.square;
    board[placedSquare] = placedPieceId;

    return {
      event: "draw",
      board,
      tray: updateTrayAfterMove(tray, move.piece),
      description: `The opponent placed piece ${placedPieceId} on square ${placedSquare}. The board is full. The game ends in a draw.`,
      analysis: {} as AnalysisResult,
    };
  }

  // Normal move - game continues
  const nextAnalysis = await analyzePosition(move.hex);
  const description =
    layer === 0
      ? `The opponent selected piece ${move.piece}.`
      : `The opponent placed piece ${parseInt(analysis.ply_grid[16], 16)} on square ${move.square} and selected piece ${move.piece}.`;

  return {
    event: "normal",
    board: getBoard(nextAnalysis.ply_grid),
    tray: updateTrayAfterMove(tray, move.piece),
    pieceToPlace: move.piece as number,
    moves: extractMovesFromAnalysis(nextAnalysis),
    description,
    analysis: nextAnalysis,
  };
}
