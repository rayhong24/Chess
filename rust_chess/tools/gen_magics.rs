use std::fs::File;
use std::io::{BufWriter, Write};
use rust_chess::game_classes::board_classes::magic_bitboard::MagicTables;
use rust_chess::game_classes::board_classes::magic_bitboard::MagicBitboard;

fn main() -> std::io::Result<()> {
    // We'll store 64 magics for bishops and rooks
    let mut bishop_magics: Vec<MagicBitboard> = Vec::with_capacity(64);
    let mut rook_magics: Vec<MagicBitboard> = Vec::with_capacity(64);

    for sq in 0..64 {
        let bishop_mask = MagicTables::bishop_mask(sq);
        let bishop_bits = bishop_mask.count_ones() as usize;
        let bishop_magic = MagicTables::find_magic(sq, bishop_bits, true);
        let bishop_shift = 64 - bishop_bits as u8;

        bishop_magics.push(MagicBitboard {
            mask: bishop_mask,
            magic: bishop_magic,
            shift: bishop_shift,
            attacks: vec![], // optional: can precompute attacks if desired
        });

        let rook_mask = MagicTables::rook_mask(sq);
        let rook_bits = rook_mask.count_ones() as usize;
        let rook_magic = MagicTables::find_magic(sq, rook_bits, false);
        let rook_shift = 64 - rook_bits as u8;

        rook_magics.push(MagicBitboard {
            mask: rook_mask,
            magic: rook_magic,
            shift: rook_shift,
            attacks: vec![], // optional: can precompute attacks if desired
        });

        println!("Square {} done", sq);
    }

    // Write to Rust file
    let file = File::create("src/game_classes/board_classes/magics_precomputed.rs")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "use crate::game_classes::board_classes::magic_bitboard::MagicBitboard;")?;
    writeln!(writer)?;
    
    writeln!(writer, "pub static BISHOP_MAGICS: [MagicBitboard; 64] = [")?;
    for m in &bishop_magics {
        writeln!(
            writer,
            "    MagicBitboard {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, attacks: vec![] }},",
            m.mask, m.magic, m.shift
        )?;
    }
    writeln!(writer, "];")?;

    writeln!(writer, "pub static ROOK_MAGICS: [MagicBitboard; 64] = [")?;
    for m in &rook_magics {
        writeln!(
            writer,
            "    MagicBitboard {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, attacks: vec![] }},",
            m.mask, m.magic, m.shift
        )?;
    }
    writeln!(writer, "];")?;

    println!("Magic numbers generated successfully!");
    Ok(())
}