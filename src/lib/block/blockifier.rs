use crate::lib::block::Block;

pub trait Blockifier<U> {
    fn next_block(&mut self) -> Blockified<U>;
    fn apply_post_update(&mut self, block: &mut Block, post_update: U) -> ();
}

pub enum Blockified<U> {
    Block { block: Block, post_update: U },
    End,
}
