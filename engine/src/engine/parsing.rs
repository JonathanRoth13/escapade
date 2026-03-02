use crate::core::{Board, DEPTH_0_SENTINEL, Node};
use anyhow::{Result, anyhow};

/// Parse a node from string representation
/// Supports two formats:
/// 1. 22-character hex string: "0123456789abcdef01234" (11 bytes as hex, no spaces)
/// 2. Grid format (17 chars): "0123456789abcde f" (16 squares + piece_to_place)
pub fn parse_node(s: &str) -> Result<Node> {
    if s.len() == 17 {
        parse_node_grid(s)
    } else {
        let trimmed = s.trim();
        if trimmed.len() == 22 {
            parse_node_hex(trimmed)
        } else {
            Err(anyhow!(
                "Invalid node string: expected 22 hex chars (binary) or exactly 17-char grid format, got {} chars",
                s.len()
            ))
        }
    }
}

/// Parse from 22 hex characters (11 bytes)
fn parse_node_hex(s: &str) -> Result<Node> {
    if s.len() != 22 {
        return Err(anyhow!("Expected 22 hex characters, got {}", s.len()));
    }

    let mut bytes = [0u8; 11];
    for i in 0..11 {
        let hex_byte = &s[i * 2..i * 2 + 2];
        bytes[i] = u8::from_str_radix(hex_byte, 16)
            .map_err(|_| anyhow!("Invalid hex byte: {}", hex_byte))?;
    }

    Ok(Node {
        board: Board::from_bytes(bytes[0..10].try_into().unwrap()),
        piece_to_place: bytes[10] >> 4, // Upper 4 bits
    })
}

/// Parse from grid format: 16 squares + piece_to_place
fn parse_node_grid(s: &str) -> Result<Node> {
    use crate::core::{LINE_MASKS, check_line_mask};

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
            continue;
        }

        empty_grid = false;

        let piece_id = ch.to_digit(16).ok_or_else(|| {
            anyhow!(
                "Invalid piece at position {}: '{}' (must be 0-F or space)",
                pos,
                ch
            )
        })? as u8;

        let bit = 1u16 << pos;
        occupancy |= bit;

        for (attr, mask) in attribute_masks.iter_mut().enumerate() {
            if (piece_id & (1 << attr)) != 0 {
                *mask |= bit;
            }
        }
    }

    let board = Board {
        occupancy,
        attribute_masks,
    };
    let has_quarto = LINE_MASKS.iter().any(|&mask| check_line_mask(&board, mask));

    if chars[16] == ' ' {
        if empty_grid {
            return Ok(DEPTH_0_SENTINEL);
        } else if has_quarto {
            return Ok(Node {
                board,
                piece_to_place: 0,
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

    Ok(Node {
        board,
        piece_to_place,
    })
}

/// Format a node as grid format (17 chars)
pub fn format_node_grid(node: &Node) -> String {
    if *node == DEPTH_0_SENTINEL {
        return "                 ".to_string();
    }

    let mut result = String::with_capacity(17);

    for pos in 0..16 {
        let bit = 1u16 << pos;
        if (node.board.occupancy & bit) != 0 {
            result.push_str(&format!("{:X}", node.board.piece_at(bit)));
        } else {
            result.push(' ');
        }
    }

    result.push_str(&format!("{:X}", node.piece_to_place));

    result
}

/// Format a node as binary hex representation (22 chars)
pub fn format_node_hex(node: &Node) -> String {
    format!(
        "{:04X}{:04X}{:04X}{:04X}{:04X}{:02X}",
        node.board.occupancy,
        node.board.attribute_masks[3],
        node.board.attribute_masks[2],
        node.board.attribute_masks[1],
        node.board.attribute_masks[0],
        node.piece_to_place << 4
    )
}
