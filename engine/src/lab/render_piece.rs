use crate::common::parse_ply;
use anyhow::Result;

// Render piece content without borders (17 chars wide, 8 lines)
fn render_piece_content(piece_id: u8) -> Vec<String> {
    // Bit encoding (matching SVG):
    // bit 0 (value 1): Hollow (1) vs Solid (0)
    // bit 1 (value 2): Short (1) vs Tall (0)
    // bit 2 (value 4): Square (1) vs Round (0)
    // bit 3 (value 8): Dark (1) vs Light (0)
    let is_tall = (piece_id & 2) == 0;
    let is_light = (piece_id & 8) == 0;
    let is_square = (piece_id & 4) != 0;
    let is_hollow = (piece_id & 1) != 0;

    let height = if is_tall { 6 } else { 3 };
    const INNER_HEIGHT: usize = 7; // Max content lines before label

    let mut piece_lines = Vec::new();

    if is_square {
        if is_light {
            for _ in 0..height {
                piece_lines.push(String::from("      █████      "));
            }
        } else {
            piece_lines.push(String::from("      ┌───┐      "));
            for _ in 1..height - 1 {
                piece_lines.push(String::from("      │   │      "));
            }
            piece_lines.push(String::from("      └───┘      "));
        }
    } else if is_light {
        piece_lines.push(String::from("      ▄███▄      "));
        for _ in 1..height - 1 {
            piece_lines.push(String::from("      █████      "));
        }
        piece_lines.push(String::from("      ▀███▀      "));
    } else {
        piece_lines.push(String::from("       ___       "));
        piece_lines.push(String::from("      /   \\      "));
        for _ in 2..height - 1 {
            piece_lines.push(String::from("      |   |      "));
        }
        piece_lines.push(String::from("      \\___/      "));
    }

    let mut inner_lines = Vec::new();
    let piece_with_indicator = if is_hollow { height + 1 } else { height };
    let top_padding = INNER_HEIGHT - piece_with_indicator;

    for _ in 0..top_padding {
        inner_lines.push(String::from("                 "));
    }

    if is_hollow {
        inner_lines.push(String::from("        ●        "));
    }

    inner_lines.extend(piece_lines);
    inner_lines.push(format!("        {:X}        ", piece_id));

    inner_lines
}

fn render_square_content(piece_id: Option<u8>) -> Vec<String> {
    if let Some(id) = piece_id {
        render_piece_content(id)
    } else {
        // Empty square - 8 lines of 17 spaces
        vec![String::from("                 "); 8]
    }
}

pub fn render_ply_display(ply_string: &str) -> Result<()> {
    // Parse the ply string
    let ply = parse_ply(ply_string)?;

    // Convert board to 2D array - extract piece IDs from occupancy and attribute masks
    let mut board = [[None; 4]; 4];
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
            let row = pos / 4;
            let col = pos % 4;
            board[row][col] = Some(piece_id);
        }
    }

    // Top border (board only for rows 3, 2, 1)
    println!("┌─────────────────┬─────────────────┬─────────────────┬─────────────────┐");

    // Render rows 3, 2, 1 (top three rows - no piece to place yet)
    for row_idx in (1..4).rev() {
        let squares: Vec<Vec<String>> = (0..4)
            .map(|col| render_square_content(board[row_idx][col]))
            .collect();

        // Print each line of this row's content
        for line_idx in 0..8 {
            print!("│");
            for (col_idx, square_lines) in squares.iter().enumerate() {
                print!("{}", square_lines[line_idx]);
                if col_idx < 3 {
                    print!("│");
                }
            }
            println!("│");
        }

        if row_idx == 1 {
            println!(
                "├─────────────────┼─────────────────┼─────────────────┼─────────────────┤   ┌─────────────────┐"
            );
        } else {
            println!("├─────────────────┼─────────────────┼─────────────────┼─────────────────┤");
        }
    }

    // Render row 0 (bottom row) with piece to place on the right
    let squares: Vec<Vec<String>> = (0..4)
        .map(|col| render_square_content(board[0][col]))
        .collect();

    // Get the piece to place content
    let piece_content = render_piece_content(ply.piece_to_place);

    for line_idx in 0..8 {
        print!("│");
        for (col_idx, square_lines) in squares.iter().enumerate() {
            print!("{}", square_lines[line_idx]);
            if col_idx < 3 {
                print!("│");
            }
        }
        println!("│   │{}│", piece_content[line_idx]);
    }

    // Board bottom border + piece box bottom
    println!(
        "└─────────────────┴─────────────────┴─────────────────┴─────────────────┘   └─────────────────┘"
    );

    Ok(())
}
