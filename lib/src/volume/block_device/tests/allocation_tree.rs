use crate::volume::block_device::allocation_tree::AllocationTree;
use crate::volume::block_device::block_group_count::BlockGroupCount;
use crate::volume::block_device::block_group_index::BlockGroupIndex;

#[test]
fn single_allocation() {
    let mut tree = AllocationTree::new(BlockGroupCount(100));

    let allocated = tree.allocate(BlockGroupCount(10)).unwrap();
    assert_eq!(allocated.block_group_count, BlockGroupCount(10));

    // Verify remaining space
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(90));
}

#[test]
fn allocate_and_deallocate() {
    let mut tree = AllocationTree::new(BlockGroupCount(100));

    // Allocate three blocks
    let _block1 = tree.allocate(BlockGroupCount(20)).unwrap();
    let block2 = tree.allocate(BlockGroupCount(30)).unwrap();
    let _block3 = tree.allocate(BlockGroupCount(40)).unwrap();
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(10));

    // Deallocate middle block
    tree.deallocate(block2);
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(40));

    // Should be able to allocate in the freed space
    let new_block = tree.allocate(BlockGroupCount(25)).unwrap();
    assert_eq!(new_block.first_block_group_index, BlockGroupIndex(20)); // Should use the gap we created
    assert_eq!(new_block.block_group_count, BlockGroupCount(25));
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(15));
}

#[test]
fn allocate_zero_blocks() {
    let mut tree = AllocationTree::new(BlockGroupCount(100));
    assert!(tree.allocate(BlockGroupCount(0)).is_none());
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(100));
}

#[test]
fn allocate_more_than_available() {
    let mut tree = AllocationTree::new(BlockGroupCount(100));
    assert!(tree.allocate(BlockGroupCount(101)).is_none());

    // Allocate most of the space
    tree.allocate(BlockGroupCount(90));

    // Try to allocate more than remaining
    assert!(tree.allocate(BlockGroupCount(11)).is_none());
}

#[test]
fn deallocate_adjacent_blocks() {
    let mut tree = AllocationTree::new(BlockGroupCount(100));

    // Allocate entire space in chunks
    let _block1 = tree.allocate(BlockGroupCount(20)).unwrap();
    let block2 = tree.allocate(BlockGroupCount(30)).unwrap();
    let block3 = tree.allocate(BlockGroupCount(50)).unwrap();
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(0));

    // Deallocate in reverse order and verify merging
    tree.deallocate(block3); // Last block
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(50));

    // Verify we can allocate the exact size of block3
    let test_block = tree.allocate(BlockGroupCount(50)).unwrap();
    assert_eq!(
        test_block.first_block_group_index,
        block3.first_block_group_index
    );
    tree.deallocate(test_block);

    // Deallocate the middle block and verify it merges with the last block
    tree.deallocate(block2);
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(80));

    // Should be able to allocate the merged space
    let merged_block = tree.allocate(BlockGroupCount(80)).unwrap();
    assert_eq!(
        merged_block.first_block_group_index,
        block2.first_block_group_index
    );
    assert_eq!(merged_block.block_group_count, BlockGroupCount(80));
}

#[test]
fn fragmentation_and_coalescing() {
    let mut tree = AllocationTree::new(BlockGroupCount(100));

    // Create fragmented allocation
    let _block1 = tree.allocate(BlockGroupCount(20)).unwrap();
    let block2 = tree.allocate(BlockGroupCount(20)).unwrap();
    let block3 = tree.allocate(BlockGroupCount(20)).unwrap();
    let block4 = tree.allocate(BlockGroupCount(20)).unwrap();
    let _block5 = tree.allocate(BlockGroupCount(20)).unwrap();
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(0));

    // Deallocate alternate blocks
    tree.deallocate(block2);
    tree.deallocate(block4);
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(40));

    // Try to allocate larger than individual fragments
    assert!(tree.allocate(BlockGroupCount(30)).is_none());

    // Deallocate block in between to test coalescing
    tree.deallocate(block3);
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(60));

    // Now we should be able to allocate a larger block
    let coalesced = tree.allocate(BlockGroupCount(40)).unwrap();
    assert_eq!(
        coalesced.first_block_group_index,
        block2.first_block_group_index
    );
    assert_eq!(coalesced.block_group_count, BlockGroupCount(40));
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(20));
}

#[test]
fn full_allocation_deallocation_cycle() {
    let mut tree = AllocationTree::new(BlockGroupCount(100));

    // Allocate full space
    let allocated = tree.allocate(BlockGroupCount(100)).unwrap();
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(0));

    // Try to allocate when full
    assert!(tree.allocate(BlockGroupCount(1)).is_none());

    // Deallocate everything
    tree.deallocate(allocated);
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(100));

    // Should be able to allocate full space again
    let reallocated = tree.allocate(BlockGroupCount(100)).unwrap();
    assert_eq!(reallocated.first_block_group_index, BlockGroupIndex(0));
    assert_eq!(reallocated.block_group_count, BlockGroupCount(100));
}

#[test]
fn test_free_block_count() {
    let mut tree = AllocationTree::new(BlockGroupCount(100));

    // Initial state should have all blocks free
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(100));

    // Allocate some blocks and check the resulting free block count
    let block1 = tree.allocate(BlockGroupCount(30)).unwrap();
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(70));

    let block2 = tree.allocate(BlockGroupCount(20)).unwrap();
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(50));

    // Deallocate and verify free block count increases
    tree.deallocate(block1);
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(80));

    // Deallocate the second block and verify all blocks are free
    tree.deallocate(block2);
    assert_eq!(tree.free_block_group_count(), BlockGroupCount(100));
}
