use crate::block::Block;

pub trait Blockifier<FinalizeData, Finalizer: BlockifierFinalizer<FinalizeData>> {
    fn next_block(&mut self) -> Blockified<FinalizeData>;
    fn into_finalizer(self) -> Finalizer;
}

pub enum Blockified<U> {
    Block { block: Block, finalize_data: U },
    End,
}

pub trait BlockifierFinalizer<U> {
    fn finalize(&mut self, block: Block, finalize_data: U) -> Block;
}
