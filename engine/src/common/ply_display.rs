use crate::common::{Board, LAYER_0_SENTINEL, Ply};
use anyhow::{Result, anyhow};

/// Parse a ply from string representation
/// Supports two formats:
/// 1. 22-character hex string: "0123456789abcdef01234" (11 bytes as hex, no spaces)
/// 2. Grid format (17 chars): "0123456789abcde f" (16 squares + piece_to_place)
///    - First 16 chars: 0-F for piece at that square, space for empty
///    - 17th char: piece_to_place (0-F)
pub fn parse_ply(s: &str) -> Result<Ply> {
    // Check for grid format first (exactly 17 chars, spaces are significant)
    if s.len() == 17 {
        // Format 2: Grid format
        parse_ply_grid(s)
    } else {
        // Try binary format after trimming whitespace
        let trimmed = s.trim();
        if trimmed.len() == 22 {
            // Format 1: 22 hex chars (11 bytes)
            parse_ply_hex(trimmed)
        } else {
            Err(anyhow!(
                "Invalid ply string: expected 22 hex chars (binary) or exactly 17-char grid format, got {} chars",
                s.len()
            ))
        }
    }
}

/// Parse from 22 hex characters (11 bytes)
fn parse_ply_hex(s: &str) -> Result<Ply> {
    if s.len() != 22 {
        return Err(anyhow!("Expected 22 hex characters, got {}", s.len()));
    }

    let mut bytes = [0u8; 11];
    for i in 0..11 {
        let hex_byte = &s[i * 2..i * 2 + 2];
        bytes[i] = u8::from_str_radix(hex_byte, 16)
            .map_err(|_| anyhow!("Invalid hex byte: {}", hex_byte))?;
    }

    // Parse using Record format
    let occupancy = u16::from_be_bytes([bytes[0], bytes[1]]);
    let a3 = u16::from_be_bytes([bytes[2], bytes[3]]);
    let a2 = u16::from_be_bytes([bytes[4], bytes[5]]);
    let a1 = u16::from_be_bytes([bytes[6], bytes[7]]);
    let a0 = u16::from_be_bytes([bytes[8], bytes[9]]);
    let piece_to_place = bytes[10] >> 4; // Upper 4 bits

    Ok(Ply {
        board: Board {
            occupancy,
            attribute_masks: [a0, a1, a2, a3],
        },
        piece_to_place,
    })
}

/// Parse from grid format: 16 squares + piece_to_place
/// Example: "0  12 3 4       f" (pieces at positions 0,3,4,5,6,9, piece_to_place=f)
fn parse_ply_grid(s: &str) -> Result<Ply> {
    use crate::common::{LINE_MASKS, check_line_mask};

    // Must be exactly 17 characters (may have leading/trailing whitespace from trim, but body must be 17)
    let chars: Vec<char> = s.chars().collect();

    if chars.len() != 17 {
        return Err(anyhow!(
            "Grid format must be exactly 17 characters (16 squares + piece), got {}",
            chars.len()
        ));
    }

    let mut empty_grid = true;

    let mut occupancy = 0u16;
    let mut attribute_masks = [0u16; 4];

    // Parse first 16 characters (the grid)
    for (pos, &ch) in chars[0..16].iter().enumerate() {
        if ch == ' ' {
            // Empty square
            continue;
        }

        empty_grid = false;

        // Parse piece ID (0-F)
        let piece_id = ch.to_digit(16).ok_or_else(|| {
            anyhow!(
                "Invalid piece at position {}: '{}' (must be 0-F or space)",
                pos,
                ch
            )
        })? as u8;

        if piece_id > 15 {
            return Err(anyhow!(
                "Piece ID must be 0-15, got {} at position {}",
                piece_id,
                pos
            ));
        }

        // Set occupancy bit
        let bit = 1u16 << pos;
        occupancy |= bit;

        // Set attribute mask bits based on piece ID
        for (attr, mask) in attribute_masks.iter_mut().enumerate() {
            if (piece_id & (1 << attr)) != 0 {
                *mask |= bit;
            }
        }
    }

    // Check if the board is terminal (has a quarto)
    let board = Board {
        occupancy,
        attribute_masks,
    };
    let has_quarto = LINE_MASKS.iter().any(|&mask| check_line_mask(&board, mask));

    if chars[16] == ' ' {
        if empty_grid {
            return Ok(LAYER_0_SENTINEL);
        } else if has_quarto {
            // Terminal position with quarto - piece_to_place should be 0 (or some sentinel)
            return Ok(Ply {
                board,
                piece_to_place: 0, // Dummy value for terminal positions
            });
        } else {
            return Err(anyhow!(
                "Invalid piece_to_place: '{}' (must be 0-F unless position is terminal)",
                chars[16]
            ));
        }
    }

    if has_quarto {
        return Err(anyhow!(
            "Terminal position (has quarto) must have piece_to_place as space, got '{}'",
            chars[16]
        ));
    }

    // Parse 17th character (piece_to_place)
    let piece_to_place = chars[16]
        .to_digit(16)
        .ok_or_else(|| anyhow!("Invalid piece_to_place: '{}' (must be 0-F)", chars[16]))?
        as u8;

    Ok(Ply {
        board,
        piece_to_place,
    })
}

/// Format a ply as grid format (17 chars)
pub fn format_ply(ply: &Ply) -> String {

    if *ply == LAYER_0_SENTINEL {
        return "                 ".to_string();
    }

    let mut result = String::with_capacity(17);

    // First 16 characters: the grid
    for pos in 0..16 {
        let bit = 1u16 << pos;
        if (ply.board.occupancy & bit) != 0 {
            // Extract piece ID from attribute masks
            let mut piece_id = 0u8;
            for (i, &mask) in ply.board.attribute_masks.iter().enumerate() {
                if (mask & bit) != 0 {
                    piece_id |= 1 << i;
                }
            }
            result.push_str(&format!("{:X}", piece_id));
        } else {
            result.push(' ');
        }
    }

    // 17th character: piece_to_place
    result.push_str(&format!("{:X}", ply.piece_to_place));

    result
}

/// Format a ply as binary hex representation (22 chars)
pub fn format_ply_hex(ply: &Ply) -> String {
    format!(
        "{:04X}{:04X}{:04X}{:04X}{:04X}{:02X}",
        ply.board.occupancy,
        ply.board.attribute_masks[3],
        ply.board.attribute_masks[2],
        ply.board.attribute_masks[1],
        ply.board.attribute_masks[0],
        ply.piece_to_place << 4 // Upper 4 bits
    )
}

/// Pretty print a ply showing the board and piece to place
pub fn pretty_print_ply(ply: &Ply) -> String {
    let mut output = String::new();

    // Build cells array with piece IDs
    let mut cells = [' '; 16];
    for (pos, cell) in cells.iter_mut().enumerate() {
        let bit = 1u16 << pos;
        if (ply.board.occupancy & bit) != 0 {
            // Extract piece ID from attribute masks
            let mut piece_id = 0u8;
            for (i, &mask) in ply.board.attribute_masks.iter().enumerate() {
                if (mask & bit) != 0 {
                    piece_id |= 1 << i;
                }
            }
            *cell = char::from_digit(piece_id as u32, 16)
                .unwrap()
                .to_ascii_uppercase();
        }
    }

    // Print board using same format as lab generate code
    output.push_str("┏━━━┯━━━┯━━━┯━━━┓\n");
    output.push_str(&format!(
        "┃ {} │ {} │ {} │ {} ┃\n",
        cells[12], cells[13], cells[14], cells[15]
    ));
    output.push_str("┠───┼───┼───┼───┨\n");
    output.push_str(&format!(
        "┃ {} │ {} │ {} │ {} ┃\n",
        cells[8], cells[9], cells[10], cells[11]
    ));
    output.push_str("┠───┼───┼───┼───┨\n");
    output.push_str(&format!(
        "┃ {} │ {} │ {} │ {} ┃\n",
        cells[4], cells[5], cells[6], cells[7]
    ));
    output.push_str("┠───┼───┼───┼───┨   ┏━━━┓\n");
    output.push_str(&format!(
        "┃ {} │ {} │ {} │ {} ┃   ┃ {:X} ┃\n",
        cells[0], cells[1], cells[2], cells[3], ply.piece_to_place
    ));
    output.push_str("┗━━━┷━━━┷━━━┷━━━┛   ┗━━━┛\n");

    //output.push_str(&format!("Piece to place: {:x}\n", ply.piece_to_place));
    //output.push_str(&format!(
    //    "Layer: {} (occupancy count)\n",
    //    ply.board.occupancy.count_ones()
    //));

    output
}
