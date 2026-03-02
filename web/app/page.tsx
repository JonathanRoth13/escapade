"use client";

import { useReducer, useState } from "react";
import Header from "./components/Header";
import LandingPage from "./components/LandingPage";
import GameBoard from "./components/GameBoard";
import PieceInHandDisplay from "./components/PieceInHandDisplay";
import PieceTray from "./components/PieceTray";
import { gameReducer, initialGameState } from "./types/game";
import type { EngineResponse } from "./types/api";
import { EMPTY_BOARD_PLY } from "./constants/gameStart";

export default function Home() {
  // Game state managed by reducer
  const [gameState, dispatch] = useReducer(gameReducer, initialGameState);

  // UI state
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<
    "overview" | "history" | "analysis"
  >("overview");

  // ============================================================================
  // Game Handlers - Play Mode
  // ============================================================================

  const startGame = async (p: "first" | "second") => {
    dispatch({ type: "START_PLAY_MODE" });

    if (p === "first") {
      dispatch({ type: "PLAYER_MOVE_FIRST" });
      return;
    }

    setLoading(true);
    try {
      const response = await fetch(
        `/api/play?ply=${encodeURIComponent(EMPTY_BOARD_PLY)}&strength=${gameState.engineStrength}`,
      );
      const data = await response.json();

      handleApiResponse(data);
    } catch (_error) {
      console.error("Error starting game:", _error);
    } finally {
      setLoading(false);
    }
  };

  const handleApiResponse = (data: EngineResponse) => {
    // Add engine's move description to history
    dispatch({ type: "ADD_MOVE_TO_HISTORY", move: data.description });

    switch (data.event) {
      case "normal":
        dispatch({
          type: "NORMAL_MOVE",
          moves: data.moves,
          board: data.board,
          tray: data.tray,
          pieceToPlace: data.pieceToPlace,
          description: data.description,
          analysis: data.analysis,
        });
        break;

      case "engine_win":
        dispatch({
          type: "ENGINE_QUARTO",
          board: data.board,
          tray: data.tray,
          highlighted: data.highlighted,
          outcome: "engine_win",
          description: data.description,
          analysis: data.analysis,
        });
        break;

      case "draw":
        dispatch({
          type: "DRAW",
          board: data.board,
          tray: data.tray,
          description: data.description,
          analysis: data.analysis,
        });
        break;
    }
  };

  const handlePieceClick = async (pieceId: number) => {
    if (!gameState.moves) {
      console.error("No moves available");
      return;
    }

    // Find matching move
    // For initial selection: match on piece only (both move.square and gameState.square are undefined)
    // After placement: match on both piece and square
    const matchingMove = gameState.moves.find((move) => {
      if (move.piece !== pieceId) return false;

      // Check if this is initial selection (no square on move) or post-placement (has square)
      const isInitialSelection = move.square === undefined;
      const hasSquareSelected = gameState.square !== undefined;

      // For initial selection, there should be no square selected yet
      if (isInitialSelection && !hasSquareSelected) return true;

      // For post-placement selection, squares must match
      if (!isInitialSelection && move.square === gameState.square) return true;

      return false;
    });

    if (!matchingMove) {
      return;
    }

    // Add player's move description to history
    dispatch({ type: "ADD_MOVE_TO_HISTORY", move: matchingMove.description });

    // Check if this move results in player win or draw (handle client-side)
    if (matchingMove.quarto) {
      // Player wins with quarto
      const board = [...gameState.board];
      if (matchingMove.square !== undefined && gameState.pieceToPlace !== undefined) {
        board[matchingMove.square] = {
          piece: gameState.pieceToPlace,
          highlighted: false,
        };
      }

      const tray = gameState.tray.map((t) => ({ ...t }));
      if (gameState.pieceToPlace !== undefined) {
        tray[gameState.pieceToPlace].available = false;
      }

      const highlighted = new Array(16).fill(false);
      const squares = matchingMove.quarto.flatMap((q) => q.intersection);
      for (let i = 0; i < 16; i++) {
        highlighted[i] = squares.includes(i);
      }

      dispatch({
        type: "PLAYER_QUARTO",
        board: board.map((cell) => cell.piece),
        tray: tray.map((t) => t.available),
        highlighted: highlighted,
        description: matchingMove.description,
        analysis: undefined,
      });
      return;
    }

    if (matchingMove.square !== undefined && matchingMove.piece === undefined) {
      // Draw - player placed last piece, no quarto
      const board = [...gameState.board];
      board[matchingMove.square] = {
        piece: gameState.pieceToPlace,
        highlighted: false,
      };

      const tray = gameState.tray.map((t) => ({ ...t }));
      if (gameState.pieceToPlace !== undefined) {
        tray[gameState.pieceToPlace].available = false;
      }

      dispatch({
        type: "DRAW",
        board: board.map((cell) => cell.piece),
        tray: tray.map((t) => t.available),
        description: matchingMove.description,
        analysis: undefined,
      });
      return;
    }

    // Normal move - call API for engine response
    setLoading(true);
    try {
      const response = await fetch(
        `/api/play?ply=${encodeURIComponent(matchingMove.hex)}&strength=${gameState.engineStrength}`,
      );
      const data = await response.json();
      handleApiResponse(data);
    } catch {
      // error handling
    } finally {
      setLoading(false);
    }

  };

  // ============================================================================
  // Board Handler
  // ============================================================================

  const handleBoardClick = async (square: number) => {
    dispatch({ type: "PLACE_PIECE", square });
  };

  const handleReturnToTitle = () => {
    dispatch({ type: "RESET" });
  };

  return (
    <div className="h-screen bg-gray-100 flex flex-col">
      <Header />
      <main
        className="flex-1 flex items-center justify-center overflow-hidden gap-[2vw]"
        style={{
          paddingLeft: "2vw",
          paddingRight: "2vw",
          paddingTop: "1.5vw",
          paddingBottom: "1.5vw",
        }}
      >
        {/* Left Panel */}
        <div
          className={
            gameState.mode !== "landing"
              ? "bg-white rounded-lg shadow-md p-6 flex flex-col gap-4"
              : ""
          }
          style={{
            width: "20vw",
            height: "min(50vw, calc(100vh - 7rem - 3vw))",
          }}
        >
          {gameState.mode !== "landing" && (
            <>
              {/* Tabs */}
              <div className="flex border-b border-gray-300 mb-4">
                {(["overview", "history", "analysis"] as const).map((tab) => (
                  <button
                    key={tab}
                    onClick={() => setActiveTab(tab)}
                    className={`flex-1 px-4 py-2 text-sm font-medium capitalize ${
                      activeTab === tab
                        ? "text-gray-900 border-b-2 border-gray-900"
                        : "text-gray-500 hover:text-gray-700"
                    }`}
                  >
                    {tab}
                  </button>
                ))}
              </div>

              {/* Tab Content */}
              {activeTab === "overview" && (
                <div className="flex-1 flex flex-col">
                  {/* Notification Area */}
                  <div className="mb-4 flex items-center justify-center">
                    <div className="space-y-2 text-center">
                      {/* Game Over States */}
                      {gameState.outcome === "player_win" && (
                        <>
                          <p className="text-4xl font-bold text-gray-900">
                            Quarto!
                          </p>
                          <p className="text-2xl font-bold text-gray-900">
                            Player Wins
                          </p>
                        </>
                      )}
                      {gameState.outcome === "engine_win" && (
                        <>
                          <p className="text-4xl font-bold text-gray-900">
                            Quarto!
                          </p>
                          <p className="text-2xl font-bold text-gray-900">
                            Engine Wins
                          </p>
                        </>
                      )}
                      {gameState.outcome === "draw" && (
                        <p className="text-4xl font-bold text-gray-900">
                          Draw
                        </p>
                      )}

                      {/* During Gameplay */}
                      {!gameState.outcome && (
                        <>
                          {!gameState.isBoardLocked ? (
                            <p className="text-3xl font-bold text-gray-800">
                              Place Piece
                            </p>
                          ) : !gameState.isRightPanelLocked ? (
                            <p className="text-3xl font-bold text-gray-800">
                              Select Piece
                            </p>
                          ) : null}
                        </>
                      )}
                    </div>
                  </div>

                  {/* Middle Section - Piece Display or Play Again */}
                  <div className="flex-1 flex items-center justify-center px-2">
                    {gameState.outcome ? (
                      <div className="border-2 border-gray-300 rounded-lg p-6 bg-white shadow-lg w-full">
                        <h3 className="text-lg font-semibold text-gray-900 mb-3 text-center">
                          Play Again
                        </h3>
                        <div className="flex flex-col gap-2">
                          <button
                            onClick={() => {
                              dispatch({ type: "RESET" });
                              startGame("first");
                            }}
                            className="w-full px-6 py-3 bg-gray-700 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
                          >
                            Go First
                          </button>
                          <button
                            onClick={() => {
                              dispatch({ type: "RESET" });
                              startGame("second");
                            }}
                            className="w-full px-6 py-3 bg-gray-700 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
                          >
                            Go Second
                          </button>
                        </div>
                      </div>
                    ) : gameState.pieceToPlace !== undefined ? (
                      <PieceInHandDisplay pieceId={gameState.pieceToPlace} />
                    ) : null}
                  </div>

                  {/* Bottom Section - Always Fixed */}
                  <div className="mt-auto">
                    {/* Engine Strength */}
                    <div className="mb-4">
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Engine Strength: {gameState.engineStrength}
                      </label>
                      <input
                        type="range"
                        min="0"
                        max="100"
                        value={gameState.engineStrength}
                        onChange={(e) =>
                          dispatch({
                            type: "SET_STRENGTH",
                            strength: Number(e.target.value),
                          })
                        }
                        className="w-full accent-gray-700"
                      />
                      <div className="flex justify-between text-xs text-gray-500 mt-1">
                        <span>Random</span>
                        <span>Perfect</span>
                      </div>
                    </div>

                    {/* Return to Title */}
                    <button
                      onClick={handleReturnToTitle}
                      className="w-full px-4 py-2 bg-gray-600 text-white rounded font-medium hover:bg-gray-700 transition-colors"
                    >
                      Return to Title Screen
                    </button>
                  </div>
                </div>
              )}

              {activeTab === "history" && (
                <div className="flex-1 mb-6 overflow-y-auto space-y-2">
                  {gameState.moveHistory.map((move, index) => (
                    <div key={index} className="flex gap-2 text-sm">
                      <span className="font-medium text-gray-700 min-w-[2rem]">
                        {index + 1}.
                      </span>
                      <span className="text-gray-600">{move}</span>
                    </div>
                  ))}
                </div>
              )}

              {activeTab === "analysis" && (
                <div className="flex-1 mb-6 overflow-y-auto">
                  {gameState.analysisData ? (
                    <div className="bg-gray-50 rounded-lg p-4 border border-gray-200">
                      <pre className="text-xs font-mono text-gray-800 whitespace-pre-wrap break-words overflow-x-auto">
                        {JSON.stringify(gameState.analysisData, null, 2)}
                      </pre>
                    </div>
                  ) : (
                    <div className="flex items-center justify-center h-full">
                      <p className="text-gray-500 text-sm text-center">
                        No analysis data available.
                        <br />
                        Make a move to see analysis.
                      </p>
                    </div>
                  )}
                </div>
              )}
            </>
          )}
        </div>

        {/* Center Panel (Square) */}
        <div
          className="flex items-center justify-center"
          style={{
            width: "min(50vw, calc(100vh - 7rem - 3vw))",
            height: "min(50vw, calc(100vh - 7rem - 3vw))",
          }}
        >
          {gameState.mode === "landing" ? (
            <LandingPage onSelectMode={startGame} />
          ) : (
            <GameBoard
              cells={gameState.board}
              enableHover={!gameState.isBoardLocked}
              loading={loading}
              onCellClick={handleBoardClick}
            />
          )}
        </div>

        {/* Right Panel */}
        <div
          className={
            gameState.mode !== "landing"
              ? "bg-white rounded-lg shadow-md p-6"
              : ""
          }
          style={{
            width: "20vw",
            height: "min(50vw, calc(100vh - 7rem - 3vw))",
          }}
        >
          {gameState.mode !== "landing" && (
            <PieceTray
              enabled={!gameState.isRightPanelLocked}
              pieces={gameState.tray}
              onPieceClick={handlePieceClick}
            />
          )}
        </div>
      </main>
    </div>
  );
}
