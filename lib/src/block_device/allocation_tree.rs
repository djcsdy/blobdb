use crate::block_device::extent::Extent;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct AllocationTree {
    free_block_count: u64,
    free_extent_block_counts_by_first_block_index: BTreeMap<u64, u64>,
    free_extent_first_block_indexes_by_block_count: BTreeMap<u64, Vec<u64>>,
}

impl AllocationTree {
    pub fn new(block_count: u64) -> Self {
        Self {
            free_block_count: block_count,
            free_extent_block_counts_by_first_block_index: BTreeMap::from([(0, block_count)]),
            free_extent_first_block_indexes_by_block_count: BTreeMap::from([(
                block_count,
                vec![0],
            )]),
        }
    }

    pub fn free_block_count(&self) -> u64 {
        self.free_block_count
    }

    pub fn allocate(&mut self, block_count: u64) -> Option<Extent> {
        if block_count == 0 {
            None
        } else if let Some(extent) = self.allocate_at_least(block_count) {
            let (extent_a, extent_b) = extent.split(block_count);
            if let Some(extent_b) = extent_b {
                self.insert(extent_b);
            }
            Some(extent_a)
        } else {
            None
        }
    }

    pub fn deallocate(&mut self, extent: Extent) {
        if extent.block_count == 0 {
            return;
        }

        let extent_before = self.extent_before(extent.first_block_index);
        let extent_after = self.extent_after(extent.first_block_index);

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

    fn allocate_at_least(&mut self, block_count: u64) -> Option<Extent> {
        if block_count == 0 {
            None
        } else if let Some(extent) = self
            .free_extent_first_block_indexes_by_block_count
            .range(block_count..)
            .next()
            .and_then(|(&block_count, first_block_indexes)| {
                first_block_indexes
                    .iter()
                    .map(|&first_block_index| Extent {
                        first_block_index,
                        block_count,
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
            .free_extent_block_counts_by_first_block_index
            .remove(&extent.first_block_index)
            != Some(extent.block_count)
        {
            panic!("Extent not found in AllocationTree")
        }
        if let Some(first_block_indexes) = self
            .free_extent_first_block_indexes_by_block_count
            .get_mut(&extent.block_count)
        {
            let mut found_count = 0;
            first_block_indexes.retain(|&first_block_index| {
                if first_block_index == extent.first_block_index {
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
                self.free_extent_first_block_indexes_by_block_count
                    .remove(&extent.block_count);
            }
        }
        self.free_block_count -= extent.block_count;
    }

    fn insert(&mut self, extent: Extent) {
        self.free_extent_block_counts_by_first_block_index
            .insert(extent.first_block_index, extent.block_count);
        if let Some(first_block_indexes) = self
            .free_extent_first_block_indexes_by_block_count
            .get_mut(&extent.block_count)
        {
            first_block_indexes.push(extent.first_block_index);
        } else {
            self.free_extent_first_block_indexes_by_block_count
                .insert(extent.block_count, vec![extent.first_block_index]);
        }
        self.free_block_count += extent.block_count
    }

    fn extent_before(&mut self, block_index: u64) -> Option<Extent> {
        self.free_extent_block_counts_by_first_block_index
            .range(..block_index)
            .map(|(&first_block_index, &block_count)| Extent {
                first_block_index,
                block_count,
            })
            .next_back()
    }

    fn extent_after(&mut self, block_index: u64) -> Option<Extent> {
        self.free_extent_block_counts_by_first_block_index
            .range(block_index..)
            .map(|(&first_block_index, &block_count)| Extent {
                first_block_index,
                block_count,
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
