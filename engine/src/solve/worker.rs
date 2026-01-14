use crate::common::{
    Board, LAYER_0_SENTINEL, OCCUPANCY_MASKS, Ply, ROOT_BOARD, Record,
    evaluate_with_remaining_pieces, hash_shard_ply, hash_worker_ply,
    is_ply_canonical_under_attribute_relabeling_only,
};
use crate::solve::worker_context::WorkerContext;
use crate::tablebase::TablebaseIndex;
use anyhow::{Result, bail};

/// Depth at which to split work among workers.
/// Lower values improve load balancing but increase hash overhead.
const SPLIT_DEPTH: usize = 4;

pub fn worker_root(
    context: &mut WorkerContext,
    total_workers: u32,
    layer: usize,
    tablebase: Option<&TablebaseIndex>,
) -> Result<()> {
    context.emit_worker_start();

    if layer > 16 {
        bail!("Invalid layer: {} (must be 0-16)", layer);
    }

    // Special cases for layers 0 and 1 (empty board)
    if layer == 0 {
        solve_layer_0(context, tablebase)?;
        context.flush_all()?;
        context.mark_mask_complete()?;
        context.emit_worker_end();
        return Ok(());
    }

    if layer == 1 {
        solve_layer_1(context, tablebase)?;
        context.flush_all()?;
        context.mark_mask_complete()?;
        context.emit_worker_end();
        return Ok(());
    }

    let split_depth = (layer - 2).min(SPLIT_DEPTH);

    let available_pieces: u16 = u16::MAX ^ 1; // piece 0 must be placed last for canonical form

    for (mask_index, &mask) in OCCUPANCY_MASKS[layer]
        .iter()
        .enumerate()
        .skip(context.current_mask())
    {
        context.set_current_mask(mask_index);

        backtrack_split(
            context,
            total_workers,
            layer,
            0,
            &ROOT_BOARD,
            mask,
            available_pieces,
            split_depth,
            tablebase,
        )?;

        context.flush_all()?;
        context.mark_mask_complete()?;
    }

    context.emit_worker_end();
    Ok(())
}

fn solve_layer_0(context: &mut WorkerContext, tablebase: Option<&TablebaseIndex>) -> Result<()> {
    // layer 0 contains only one position, namely the root position (board empty, piece not yet
    // selected), which we represent with a sentinel
    if context.worker_id != 0 {
        return Ok(());
    }

    let all_pieces_except_0: u16 = u16::MAX ^ 1; // piece 0 must be placed last for canonical form

    // the only canonical move in layer 1 is this one, so the outcome of layer 0 is the outcome of
    let outcome = evaluate_with_remaining_pieces(
        &Ply {
            board: ROOT_BOARD,
            piece_to_place: 0,
        },
        all_pieces_except_0,
        tablebase,
    );

    let record = Record {
        ply: LAYER_0_SENTINEL,
        outcome,
    };
    let key = record.to_bytes();
    let shard_hash = hash_shard_ply(&LAYER_0_SENTINEL);
    let shard_id = context.shard_id_from_hash(shard_hash);
    context.append(shard_id, &key)?;
    Ok(())
}

/// Solve layer 1: empty board, canonical piece to place = 0
fn solve_layer_1(context: &mut WorkerContext, tablebase: Option<&TablebaseIndex>) -> Result<()> {
    if context.worker_id != 0 {
        return Ok(());
    }
    // layer 1 contains sixteen positions, one for each piece that has been seleceted
    // only the 0 piece is canonical
    let ply = Ply {
        board: ROOT_BOARD,
        piece_to_place: 0,
    };
    let all_pieces_except_0: u16 = u16::MAX ^ 1;
    let record = Record {
        ply,
        outcome: crate::common::evaluate_with_remaining_pieces(
            &ply,
            all_pieces_except_0,
            tablebase,
        ),
    };
    let key = record.to_bytes();
    let shard_hash = hash_shard_ply(&ply);
    let shard_id = context.shard_id_from_hash(shard_hash);
    context.append(shard_id, &key)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn backtrack_split(
    context: &mut WorkerContext,
    total_workers: u32,
    layer: usize,
    depth: usize,
    board: &Board,
    available_squares: u16,
    available_pieces: u16,
    split_depth: usize,
    tablebase: Option<&TablebaseIndex>,
) -> Result<()> {
    if depth == split_depth {
        let ply: Ply = Ply {
            board: *board,
            piece_to_place: 0,
        };
        let worker_hash = hash_worker_ply(&ply);
        if (worker_hash % total_workers as u64) != context.worker_id as u64 {
            return Ok(());
        }
    }

    // we have one piece remaining to place
    if depth == layer - 2 {
        // move to depth layer - 1
        let (final_board, is_quarto) =
            crate::common::check_after_place(board, 0, available_squares);

        if is_quarto {
            return Ok(());
        }

        let mut running_pieces_available: u16 = available_pieces;
        while running_pieces_available != 0 {
            let piece: u16 = lowest_bit(running_pieces_available);
            running_pieces_available ^= piece;
            let piece_index: u8 = piece.trailing_zeros() as u8;
            let ply = Ply {
                board: final_board,
                piece_to_place: piece_index,
            };
            if !is_ply_canonical_under_attribute_relabeling_only(&ply) {
                continue;
            }

            let remaining_pieces = available_pieces ^ piece;
            let outcome: u8 =
                crate::common::evaluate_with_remaining_pieces(&ply, remaining_pieces, tablebase);

            let shard_hash = hash_shard_ply(&ply);
            let record = Record { ply, outcome };
            let key = record.to_bytes();
            let shard_id = context.shard_id_from_hash(shard_hash);
            context.append(shard_id, &key)?;
        }

        return Ok(());
    }

    let square: u16 = lowest_bit(available_squares);
    let mut running_pieces_available: u16 = available_pieces;

    while running_pieces_available != 0 {
        let piece: u16 = lowest_bit(running_pieces_available);
        running_pieces_available ^= piece;
        let piece_index: u8 = piece.trailing_zeros() as u8;

        let (board_next, is_quarto) = crate::common::check_after_place(board, piece_index, square);

        if is_quarto {
            continue;
        }

        let available_squares_next: u16 = available_squares ^ square;
        let available_pieces_next: u16 = available_pieces ^ piece;

        backtrack_split(
            context,
            total_workers,
            layer,
            depth + 1,
            &board_next,
            available_squares_next,
            available_pieces_next,
            split_depth,
            tablebase,
        )?;
    }

    Ok(())
}

#[inline(always)]
fn lowest_bit(mask: u16) -> u16 {
    mask & mask.wrapping_neg()
}
