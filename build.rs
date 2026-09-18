//! Generates compact two-level lookup tables for `no_accent_char` and
//! `lower_no_accent_char`.
//!
//! Level 1 maps `code_point >> 8` to a block id. Each block holds 256 `u16`
//! codes for the low byte: `EMPTY`, `IDENTITY` or an index into `ENTRIES`,
//! which packs `(offset << 8) | len` into a shared string `POOL`.
//! Blocks are deduplicated, so unassigned / identity-only ranges cost nothing.
//! Hangul syllables (U+AC00..=U+D7A3) are left out: their decomposition is
//! arithmetic and handled in `Table::lookup`.

use std::collections::HashMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

const EMPTY: u16 = 0;
const IDENTITY: u16 = 1;
const FIRST_ENTRY: u16 = 2;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    generate(&out_dir.join("no_accent_map.rs"), |c, out| {
        out.extend(c.nfd().filter(keep));
    });

    generate(&out_dir.join("lower_no_accent_map.rs"), |c, out| {
        out.extend(c.to_lowercase().nfd().filter(keep));
    });

    println!("cargo:rerun-if-changed=build.rs");
}

fn keep(c: &char) -> bool {
    c.is_ascii() || c.is_alphanumeric()
}

fn generate(path: &Path, map: impl Fn(char, &mut String)) {
    let mut pool = String::new();
    let mut entries: Vec<u32> = Vec::new();
    let mut entry_ids: HashMap<String, u16> = HashMap::new();
    let mut blocks: Vec<[u16; 256]> = Vec::new();
    let mut block_ids: HashMap<[u16; 256], u16> = HashMap::new();
    let mut level1: Vec<u16> = Vec::with_capacity(0x1100);
    let mut mapped = String::new();

    for hi in 0..0x1100u32 {
        let mut block = [EMPTY; 256];

        for (lo, slot) in block.iter_mut().enumerate() {
            let cp = (hi << 8) | lo as u32;

            if (0xAC00..=0xD7A3).contains(&cp) {
                continue;
            }

            let Some(c) = char::from_u32(cp) else {
                continue;
            };

            mapped.clear();
            map(c, &mut mapped);

            *slot = if mapped.is_empty() {
                EMPTY
            } else if mapped.chars().eq(std::iter::once(c)) {
                IDENTITY
            } else if let Some(&id) = entry_ids.get(&mapped) {
                id
            } else {
                let id = u16::try_from(FIRST_ENTRY as usize + entries.len())
                    .expect("too many distinct mappings for u16");
                let offset = u32::try_from(pool.len()).unwrap();
                let len = u32::try_from(mapped.len()).unwrap();
                assert!(offset < (1 << 24) && len < (1 << 8), "pool too large");

                entries.push((offset << 8) | len);
                pool.push_str(&mapped);
                entry_ids.insert(mapped.clone(), id);
                id
            };
        }

        let block_id = *block_ids.entry(block).or_insert_with(|| {
            blocks.push(block);
            u16::try_from(blocks.len() - 1).expect("too many blocks for u16")
        });

        level1.push(block_id);
    }

    let flat_blocks: Vec<u16> = blocks.iter().flatten().copied().collect();

    let mut src = String::new();
    writeln!(
        src,
        "pub(super) static LEVEL1: [u16; {}] = {:?};",
        level1.len(),
        level1
    )
    .unwrap();
    writeln!(
        src,
        "pub(super) static BLOCKS: [u16; {}] = {:?};",
        flat_blocks.len(),
        flat_blocks
    )
    .unwrap();
    writeln!(
        src,
        "pub(super) static ENTRIES: [u32; {}] = {:?};",
        entries.len(),
        entries
    )
    .unwrap();
    writeln!(src, "pub(super) static POOL: &str = {:?};", pool).unwrap();

    fs::write(path, src).unwrap();
}
