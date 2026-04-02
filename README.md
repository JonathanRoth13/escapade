# Escapade

A Quarto engine that solves the complete game tree using tablebase generation for sub-microsecond lookups. Written in Rust for performance and ergonomic bit-array representations of the board. [Play it here.](https://roth13.com) You can run this locally using `docker compose up`.

## Quarto

Quarto is played on a 4×4 board with 16 unique pieces. Each piece is tall or short, light or dark, square or circular, and hollow or solid. Players take turns choosing a piece which their opponent must place. Win by completing a line of four pieces that share any attribute.

## The Game Tree

A **game tree** represents every possible sequence of moves in a game. The root is the starting position, and each node branches into all legal moves from that state.

- **Depth 0**: Empty board, no piece selected yet
- **Depth 1**: Empty board, one piece selected
- **Depth 2–16**: Board with N-1 pieces placed, one piece selected (unless quarto)
- **Depth 17**: All 16 pieces placed. Draw (unless quarto)

## Tablebases

Quarto has a game tree of ~2.7 trillion positions. While minimax with alpha-beta pruning works for shallow positions, calculating outcomes below depth 7 becomes prohibitively slow without precomputed data. A single query at depth 3 can take minutes.

The solution: generate databases mapping every position to its precomputed outcome, so that any position resolves in O(1) time via lookup. Such databases are known as tablebases. The challenge is making lookups fast without wasting space, because ideally, tablebases are loaded into memory. To accomplish this, I used a minimal perfect hash (MPH), specifically the [Compress, Hash, Displace algorithm](https://cmph.sourceforge.net/chd.html).

## Tablebase Generation
```
┌─────────────────────────────────────────────────────┐
│                    SOLVE                            │
│  Generate all canonical positions for a depth       │
│  Evaluate via negamax (use previous depth's         │
│    tablebase for cutoff, if available)              │
│  Output: 11B key-value pairs (position → outcome)   │
└─────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────┐
│                    MERGE                            │
│  Concatenate into shard files                       │
│  Output: shard files                                │
└─────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────┐
│                    INDEX                            │
│  Build minimal perfect hash over shard file         │
│  Output: tablebase file                             │
└─────────────────────────────────────────────────────┘

              ↻ repeat for depths 10 → 0
```

These are subcommands of the `escapade` CLI (`escapade solve`, `escapade merge`, `escapade index`).

Solve writes output incrementally to many small files, enabling resume capability (critical for EC2 spot instances). Merge consolidates these into larger shard files for indexing.

### Bootstrapping

The first depth I could solve from scratch was depth 10. With 89 billion nodes, it took roughly a week on a 32-core EC2 instance. This created roughly 1 TiB of data, which was partitioned into 8 shard files, each roughly 128 GiB. This was necessary because the EC2 instance only has 244 GiB of RAM, and the entire shard needs to be loaded into memory for the index step. A tablebase was generated for each of these shards.

Once depth 10 was indexed, I solved depth 9. Positions at depth 9 only need to search one level deep before hitting the depth 10 tablebase, reducing evaluation from many seconds to microseconds. This meant depth 9, with 16 billion positions, solved orders of magnitude faster than the previous, only a few hours. This process was repeated so that tablebases were generated for depth 0 through 10.

## Web App

The engine runs as an HTTP server (`escapade engine`) returning JSON with every legal move and its outcome. A Next.js app communicates with it over the network. When the frontend sends a game state, the backend forwards it to the engine, receives the full move analysis, picks the AI's move, and computes the render state for the board. The two services run as separate containers via Docker Compose.
