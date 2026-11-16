/**
 * Attribute names indexed by attribute id, matching the engine:
 * id = 2 * bit + value, where `bit` is the piece bit (0 hollow/solid,
 * 1 short/tall, 2 square/round, 3 dark/light) and `value` is 1 when the
 * piece has that bit set.
 */
export const ATTRIBUTE_NAMES = [
  "solid",
  "hollow",
  "tall",
  "short",
  "round",
  "square",
  "light",
  "dark",
] as const;

export function pieceAttributes(piece: number): string[] {
  return [0, 1, 2, 3].map(
    (bit) => ATTRIBUTE_NAMES[2 * bit + ((piece >> bit) & 1)],
  );
}
