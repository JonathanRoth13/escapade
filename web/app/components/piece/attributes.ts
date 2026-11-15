export function pieceAttributes(piece: number): string[] {
  return [
    piece & 2 ? "short" : "tall",
    piece & 8 ? "dark" : "light",
    piece & 4 ? "square" : "round",
    piece & 1 ? "hollow" : "solid",
  ];
}
