use crate::lib::block::Block;

pub trait Blockifier<PostUpdate, PostUpdater: BlockifierPostUpdater<PostUpdate>> {
    fn next_block(&mut self) -> Blockified<PostUpdate>;
    fn into_post_updater(self) -> PostUpdater;
}

pub enum Blockified<U> {
    Block { block: Block, post_update: U },
    End,
}

pub trait BlockifierPostUpdater<U> {
    fn apply_post_update(&mut self, block: Block, post_update: U) -> Block;
}
