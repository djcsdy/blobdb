use crate::units::BlockGroupCount;
use crate::units::BlockGroupIndex;
use crate::volume::block_device::extent::Extent;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct AllocationTree {
    free_count: BlockGroupCount,
    free_extent_block_group_counts_by_first_block_group_index:
        BTreeMap<BlockGroupIndex, BlockGroupCount>,
    free_extent_first_block_group_indexes_by_block_group_count:
        BTreeMap<BlockGroupCount, Vec<BlockGroupIndex>>,
}

impl AllocationTree {
    pub fn new(block_group_count: BlockGroupCount) -> Self {
        Self {
            free_count: block_group_count,
            free_extent_block_group_counts_by_first_block_group_index: BTreeMap::from([(
                BlockGroupIndex(0),
                block_group_count,
            )]),
            free_extent_first_block_group_indexes_by_block_group_count: BTreeMap::from([(
                block_group_count,
                vec![BlockGroupIndex(0)],
            )]),
        }
    }

    pub fn free_block_group_count(&self) -> BlockGroupCount {
        self.free_count
    }

    pub fn allocate(&mut self, block_group_count: BlockGroupCount) -> Option<Extent> {
        if block_group_count == BlockGroupCount(0) {
            None
        } else if let Some(extent) = self.allocate_at_least(block_group_count) {
            let (extent_a, extent_b) = extent.split(block_group_count);
            if let Some(extent_b) = extent_b {
                self.insert(extent_b);
            }
            Some(extent_a)
        } else {
            None
        }
    }

    pub fn deallocate(&mut self, extent: Extent) {
        if extent.block_group_count == BlockGroupCount(0) {
            return;
        }

        let extent_before = self.extent_before(extent.first_block_group_index);
        let extent_after = self.extent_after(extent.first_block_group_index);

        if extent_before
            .map(|before| before.overlaps(extent))
            .unwrap_or(false)
            || extent_after
                .map(|after| after.overlaps(extent))
                .unwrap_or(false)
        {
            panic!("Cannot deallocate overlapping extent")
        }

        let extent = self.merge_if_adjacent(extent, extent_before);
        let extent = self.merge_if_adjacent(extent, extent_after);

        self.insert(extent)
    }

    fn allocate_at_least(&mut self, block_group_count: BlockGroupCount) -> Option<Extent> {
        if block_group_count == BlockGroupCount(0) {
            None
        } else if let Some(extent) = self
            .free_extent_first_block_group_indexes_by_block_group_count
            .range(block_group_count..)
            .next()
            .and_then(|(&block_count, first_block_indexes)| {
                first_block_indexes
                    .iter()
                    .map(|&first_block_index| Extent {
                        first_block_group_index: first_block_index,
                        block_group_count: block_count,
                    })
                    .next()
            })
        {
            self.remove(extent);
            Some(extent)
        } else {
            None
        }
    }

    fn remove(&mut self, extent: Extent) {
        if self
            .free_extent_block_group_counts_by_first_block_group_index
            .remove(&extent.first_block_group_index)
            != Some(extent.block_group_count)
        {
            panic!("Extent not found in AllocationTree")
        }
        if let Some(first_block_indexes) = self
            .free_extent_first_block_group_indexes_by_block_group_count
            .get_mut(&extent.block_group_count)
        {
            let mut found_count = 0;
            first_block_indexes.retain(|&first_block_index| {
                if first_block_index == extent.first_block_group_index {
                    found_count += 1;
                    false
                } else {
                    true
                }
            });
            if found_count != 1 {
                panic!("Expected exactly one matching first_block_index in AllocationTree");
            }
            if first_block_indexes.is_empty() {
                self.free_extent_first_block_group_indexes_by_block_group_count
                    .remove(&extent.block_group_count);
            }
        }
        self.free_count -= extent.block_group_count;
    }

    fn insert(&mut self, extent: Extent) {
        self.free_extent_block_group_counts_by_first_block_group_index
            .insert(extent.first_block_group_index, extent.block_group_count);
        if let Some(first_block_indexes) = self
            .free_extent_first_block_group_indexes_by_block_group_count
            .get_mut(&extent.block_group_count)
        {
            first_block_indexes.push(extent.first_block_group_index);
        } else {
            self.free_extent_first_block_group_indexes_by_block_group_count
                .insert(
                    extent.block_group_count,
                    vec![extent.first_block_group_index],
                );
        }
        self.free_count += extent.block_group_count
    }

    fn extent_before(&mut self, block_group_index: BlockGroupIndex) -> Option<Extent> {
        self.free_extent_block_group_counts_by_first_block_group_index
            .range(..block_group_index)
            .map(|(&first_block_index, &block_count)| Extent {
                first_block_group_index: first_block_index,
                block_group_count: block_count,
            })
            .next_back()
    }

    fn extent_after(&mut self, block_group_index: BlockGroupIndex) -> Option<Extent> {
        self.free_extent_block_group_counts_by_first_block_group_index
            .range(block_group_index..)
            .map(|(&first_block_index, &block_count)| Extent {
                first_block_group_index: first_block_index,
                block_group_count: block_count,
            })
            .next()
    }

    fn merge_if_adjacent(&mut self, new: Extent, existing: Option<Extent>) -> Extent {
        existing
            .and_then(|existing| {
                existing.merge(new).map(|merged| {
                    self.remove(existing);
                    merged
                })
            })
            .unwrap_or(new)
    }
}
