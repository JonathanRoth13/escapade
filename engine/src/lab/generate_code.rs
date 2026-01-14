use crate::common::{Transformation, apply_u16, is_canonical_occupancy};

/// Find all transformations that produce the canonical occupancy for a given occupancy
fn get_canonical_transformations(occupancy: u16) -> Vec<Transformation> {
    let mut best_occupancy = occupancy;
    let mut best_transformations: Vec<Transformation> = vec![Transformation::Identity];

    for &t in Transformation::ALL.iter().skip(1) {
        let transformed = apply_u16(occupancy, t);

        match transformed.cmp(&best_occupancy) {
            std::cmp::Ordering::Less => {
                best_occupancy = transformed;
                best_transformations = vec![t];
            }
            std::cmp::Ordering::Equal => {
                best_transformations.push(t);
            }
            std::cmp::Ordering::Greater => {}
        }
    }

    best_transformations
}

/// Convert a list of transformations to a bitmask for naming
fn transformations_to_bitmask(transformations: &[Transformation]) -> u8 {
    let mut mask = 0u8;
    for &t in transformations {
        let bit = match t {
            Transformation::Identity => 0,
            Transformation::Rotate90 => 1,
            Transformation::Rotate180 => 2,
            Transformation::Rotate270 => 3,
            Transformation::ReflectHorizontal => 4,
            Transformation::ReflectVertical => 5,
            Transformation::ReflectDiagonalMain => 6,
            Transformation::ReflectDiagonalAnti => 7,
        };
        mask |= 1 << bit;
    }
    mask
}

/// Print a visual representation of a 16-bit board mask
fn print_board_visual(mask: u16, indent: &str) {
    let mut cells = [' '; 16];
    for (i, cell) in cells.iter_mut().enumerate() {
        *cell = if ((mask >> i) & 1) != 0 { 'X' } else { ' ' };
    }

    println!("{indent}// ┏━━━┯━━━┯━━━┯━━━┓");
    println!(
        "{indent}// ┃ {} │ {} │ {} │ {} ┃",
        cells[12], cells[13], cells[14], cells[15]
    );
    println!("{indent}// ┠───┼───┼───┼───┨");
    println!(
        "{indent}// ┃ {} │ {} │ {} │ {} ┃",
        cells[8], cells[9], cells[10], cells[11]
    );
    println!("{indent}// ┠───┼───┼───┼───┨");
    println!(
        "{indent}// ┃ {} │ {} │ {} │ {} ┃",
        cells[4], cells[5], cells[6], cells[7]
    );
    println!("{indent}// ┠───┼───┼───┼───┨");
    println!(
        "{indent}// ┃ {} │ {} │ {} │ {} ┃",
        cells[0], cells[1], cells[2], cells[3]
    );
    println!("{indent}// ┗━━━┷━━━┷━━━┷━━━┛");
}

/// Generate canonical occupancy masks for each layer
/// Layers 0-1: 0 pieces on board
/// Layers 2-17: layer-1 pieces on board
pub fn generate_occupancy_masks() {
    let mut piece_count_to_masks: Vec<(u32, Vec<u16>)> = Vec::with_capacity(17);

    // Collect canonical occupancy patterns for each piece count (0-16 pieces)
    for pieces_on_board in 0u32..=16u32 {
        let mut masks = Vec::new();
        for candidate in 0u32..(1u32 << 16) {
            if candidate.count_ones() != pieces_on_board {
                continue;
            }
            let mask = candidate as u16;
            if is_canonical_occupancy(mask) {
                masks.push(mask);
            }
        }
        piece_count_to_masks.push((pieces_on_board, masks));
    }

    // Print array constants for each piece count (0-16 pieces)
    for (pieces, masks) in &piece_count_to_masks {
        println!(
            "pub const OCCUPANCY_{pieces:02}: [u16; {}] = [",
            masks.len()
        );

        for &mask in masks {
            print_board_visual(mask, "    ");
            println!("    0x{mask:04X},");
        }
        println!("];");
        println!();
    }

    // Print index array (18 entries: layer 0-17)
    // Layers 0 and 1: 0 pieces on board
    // Layers 2-17: layer-1 pieces on board
    print!("pub const INDEX: [&[u16]; 18] = [");
    for layer in 0u32..=17u32 {
        if layer > 0 {
            print!(" ");
        }
        let pieces_on_board = layer.saturating_sub(1);
        print!("&OCCUPANCY_{pieces_on_board:02},");
    }
    println!("];");
}

/// Generate line mask constants (rows, columns, diagonals)
pub fn generate_line_masks() {
    let masks = [
        ("ROW_0", 0b0000_0000_0000_1111u16),
        ("ROW_1", 0b0000_0000_1111_0000u16),
        ("ROW_2", 0b0000_1111_0000_0000u16),
        ("ROW_3", 0b1111_0000_0000_0000u16),
        ("COL_0", 0b0001_0001_0001_0001u16),
        ("COL_1", 0b0010_0010_0010_0010u16),
        ("COL_2", 0b0100_0100_0100_0100u16),
        ("COL_3", 0b1000_1000_1000_1000u16),
        ("DIAG_MAIN", 0b1000_0100_0010_0001u16),
        ("DIAG_ANTI", 0b0001_0010_0100_1000u16),
    ];

    for (name, mask) in masks {
        print_board_visual(mask, "");
        println!("pub const {name}: u16 = 0x{mask:04X};\n");
    }

    // Print LINE_MASKS array
    println!("pub const LINE_MASKS: [u16; 10] = [");
    println!("    ROW_0, ROW_1, ROW_2, ROW_3, COL_0, COL_1, COL_2, COL_3, DIAG_MAIN, DIAG_ANTI,");
    println!("];");
}

/// Generate canonical transformation lookup table for all 2^16 occupancy patterns
pub fn generate_canonical_transformations() {
    use std::collections::BTreeMap;

    println!("use crate::common::transform::Transformation;\n");
    println!("// Transformation arrays named by bitmask:");
    println!("// Bit 0 = Identity, Bit 1 = Rotate90, Bit 2 = Rotate180, Bit 3 = Rotate270");
    println!("// Bit 4 = ReflectHorizontal, Bit 5 = ReflectVertical");
    println!("// Bit 6 = ReflectDiagonalMain, Bit 7 = ReflectDiagonalAnti");
    println!("// Example: TRANS_0xFF = all 8 transformations (full D8 symmetry)");
    println!("//          TRANS_0x0F = all 4 rotations (C4 rotational symmetry)");
    println!("//          TRANS_0x01 = Identity only (no symmetry)\n");

    // Collect all unique transformation lists by their bitmask
    let mut unique_lists: BTreeMap<u8, Vec<Transformation>> = BTreeMap::new();
    let mut occupancy_to_bitmask: Vec<u8> = Vec::with_capacity(65536);

    for occupancy in 0u32..65536u32 {
        let transformations = get_canonical_transformations(occupancy as u16);
        let bitmask = transformations_to_bitmask(&transformations);

        unique_lists.entry(bitmask).or_insert(transformations);
        occupancy_to_bitmask.push(bitmask);
    }

    // Generate const arrays for each unique transformation list (sorted by bitmask)
    for (bitmask, transformations) in &unique_lists {
        print!(
            "const TRANSFORMATION_{:02X}: [Transformation; {}] = [",
            bitmask,
            transformations.len()
        );
        for (i, t) in transformations.iter().enumerate() {
            print!("Transformation::{:?}", t);
            if i < transformations.len() - 1 {
                print!(", ");
            }
        }
        println!("];");
    }

    println!();

    // Generate the main lookup table
    println!("pub static CANONICAL_TRANSFORMATIONS: [&[Transformation]; 65536] = [");

    for (i, bitmask) in occupancy_to_bitmask.iter().enumerate() {
        if i % 4 == 0 {
            print!("    ");
        }

        print!("&TRANSFORMATION_{:02X}", bitmask);

        if i < 65535 {
            print!(", ");
        }

        if i % 4 == 3 {
            println!();
        }
    }

    println!("];");
}
