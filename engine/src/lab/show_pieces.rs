use anyhow::Result;

pub fn run() -> Result<()> {
    println!("┏━━━━━━┯━━━━━━┯━━━━━━┯━━━━━━━┯━━━━━━━┓");
    println!("┃ Piece│Height│ Color│ Shape │  Top  ┃");
    println!("┠──────┼──────┼──────┼───────┼───────┨");

    for piece_id in 0..16 {
        let height = if (piece_id & 2) != 0 { "Short" } else { "Tall" };
        let color = if (piece_id & 8) != 0 { "Dark" } else { "Light" };
        let shape = if (piece_id & 4) != 0 {
            "Square"
        } else {
            "Round"
        };
        let top = if (piece_id & 1) != 0 {
            "Hollow"
        } else {
            "Solid"
        };

        println!(
            "┃  {:X}   │{:^6}│{:^6}│{:^7}│{:^7}┃",
            piece_id, height, color, shape, top
        );
    }

    println!("┗━━━━━━┷━━━━━━┷━━━━━━┷━━━━━━━┷━━━━━━━┛");

    Ok(())
}
