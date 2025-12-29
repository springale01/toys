#![allow(unused)]

use crate::blocks::{Block, Blocks};

mod blocks;
fn main() {
    let mut blocks = Blocks::new();
    let joe = Joe;
    let coolblock = Block::Beneficial {
        value: 0,
        affect: 10,
    };
    let evilblock = Block::Harmful {
        value: 0,
        affect: -10,
    };
    let normie = Block::Normie { value: 0 };
    let coolblock2 = Block::Beneficial {
        value: 0,
        affect: 5,
    };
    let blockz = vec![normie, evilblock, coolblock, coolblock2];

    if joe.exists() {
        blocks.blocks = blockz;
        println!("{}", blocks.to_string());
        blocks.affect_other_blocks_v2();
        print!("\n");
        println!("{}", blocks.to_string())
    }
}

struct Joe;

trait Exists {
    fn exists(&self) -> bool;
}

impl Exists for Joe {
    fn exists(&self) -> bool {
        true
    }
}
