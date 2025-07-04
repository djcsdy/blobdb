use crate::units::BlockGroupCount;
use crate::units::BlockGroupIndex;
use crate::volume::block_device::extent::Extent;

#[test]
fn end_block_group_index() {
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(5),
        block_group_count: BlockGroupCount(3),
    };
    assert_eq!(extent.end_block_group_index(), BlockGroupIndex(8));

    let extent = Extent {
        first_block_group_index: BlockGroupIndex(0),
        block_group_count: BlockGroupCount(1),
    };
    assert_eq!(extent.end_block_group_index(), BlockGroupIndex(1));
}

#[test]
fn split() {
    // Test normal split
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let (first, second) = extent.split(BlockGroupCount(3));
    assert_eq!(
        first,
        Extent {
            first_block_group_index: BlockGroupIndex(10),
            block_group_count: BlockGroupCount(3),
        }
    );
    assert_eq!(
        second,
        Some(Extent {
            first_block_group_index: BlockGroupIndex(13),
            block_group_count: BlockGroupCount(2),
        })
    );

    // Test split at exact size
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let (first, second) = extent.split(BlockGroupCount(5));
    assert_eq!(first, extent);
    assert_eq!(second, None);
}

#[test]
#[should_panic(
    expected = "block_group_count must be less than or equal to extent block_group_count"
)]
fn split_panic() {
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    extent.split(BlockGroupCount(6)); // Should panic
}

#[test]
fn merge() {
    // Test successful merge
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(15),
        block_group_count: BlockGroupCount(3),
    };
    assert_eq!(
        extent1.merge(extent2),
        Some(Extent {
            first_block_group_index: BlockGroupIndex(10),
            block_group_count: BlockGroupCount(8),
        })
    );

    // Test merge with gap
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(16),
        block_group_count: BlockGroupCount(3),
    };
    assert_eq!(extent1.merge(extent2), None);

    // Test merge with reversed order
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(15),
        block_group_count: BlockGroupCount(3),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    assert_eq!(
        extent1.merge(extent2),
        Some(Extent {
            first_block_group_index: BlockGroupIndex(10),
            block_group_count: BlockGroupCount(8),
        })
    );
}

#[test]
fn overlaps() {
    // Test overlapping extents
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(13),
        block_group_count: BlockGroupCount(3),
    };
    assert!(extent1.overlaps(extent2));
    assert!(extent2.overlaps(extent1));

    // Test adjacent but non-overlapping extents
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(15),
        block_group_count: BlockGroupCount(3),
    };
    assert!(!extent1.overlaps(extent2));
    assert!(!extent2.overlaps(extent1));

    // Test completely separate extents
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(20),
        block_group_count: BlockGroupCount(3),
    };
    assert!(!extent1.overlaps(extent2));
    assert!(!extent2.overlaps(extent1));

    // Test one extent completely containing another
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(10),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(12),
        block_group_count: BlockGroupCount(3),
    };
    assert!(extent1.overlaps(extent2));
    assert!(extent2.overlaps(extent1));
}

#[test]
fn contains() {
    // Test complete containment
    let outer = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(10),
    };
    let inner = Extent {
        first_block_group_index: BlockGroupIndex(12),
        block_group_count: BlockGroupCount(3),
    };
    assert!(outer.contains(inner));
    assert!(!inner.contains(outer));

    // Test extent contains itself
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    assert!(extent.contains(extent));

    // Test partial overlap (should not contain)
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(13),
        block_group_count: BlockGroupCount(5),
    };
    assert!(!extent1.contains(extent2));
    assert!(!extent2.contains(extent1));

    // Test completely separate extents
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(20),
        block_group_count: BlockGroupCount(3),
    };
    assert!(!extent1.contains(extent2));
    assert!(!extent2.contains(extent1));

    // Test adjacent extents
    let extent1 = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let extent2 = Extent {
        first_block_group_index: BlockGroupIndex(15),
        block_group_count: BlockGroupCount(3),
    };
    assert!(!extent1.contains(extent2));
    assert!(!extent2.contains(extent1));

    // Test containment at boundaries
    let outer = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    let inner = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(3),
    };
    assert!(outer.contains(inner));
    assert!(!inner.contains(outer));

    let inner2 = Extent {
        first_block_group_index: BlockGroupIndex(12),
        block_group_count: BlockGroupCount(3),
    };
    assert!(outer.contains(inner2));
    assert!(!inner2.contains(outer));
}

#[test]
fn reserve() {
    // Test reserving in the middle
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(10),
    };
    let (before, after) = extent.reserve(Extent {
        first_block_group_index: BlockGroupIndex(13),
        block_group_count: BlockGroupCount(4),
    });

    assert_eq!(
        before,
        Some(Extent {
            first_block_group_index: BlockGroupIndex(10),
            block_group_count: BlockGroupCount(3),
        })
    );
    assert_eq!(
        after,
        Some(Extent {
            first_block_group_index: BlockGroupIndex(17),
            block_group_count: BlockGroupCount(3),
        })
    );

    // Test reserving at the start
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(10),
    };
    let (before, after) = extent.reserve(Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(4),
    });

    assert_eq!(before, None);
    assert_eq!(
        after,
        Some(Extent {
            first_block_group_index: BlockGroupIndex(14),
            block_group_count: BlockGroupCount(6),
        })
    );

    // Test reserving at the end
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(10),
    };
    let (before, after) = extent.reserve(Extent {
        first_block_group_index: BlockGroupIndex(16),
        block_group_count: BlockGroupCount(4),
    });

    assert_eq!(
        before,
        Some(Extent {
            first_block_group_index: BlockGroupIndex(10),
            block_group_count: BlockGroupCount(6),
        })
    );
    assert_eq!(after, None);

    // Test reserving the whole extent
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(10),
    };
    let (before, after) = extent.reserve(extent);

    assert_eq!(before, None);
    assert_eq!(after, None);
}

#[test]
#[should_panic(expected = "does not contain")]
fn reserve_not_contained() {
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    extent.reserve(Extent {
        first_block_group_index: BlockGroupIndex(8),
        block_group_count: BlockGroupCount(3),
    }); // should panic
}

#[test]
#[should_panic(expected = "does not contain")]
fn reserve_partially_contained() {
    let extent = Extent {
        first_block_group_index: BlockGroupIndex(10),
        block_group_count: BlockGroupCount(5),
    };
    extent.reserve(Extent {
        first_block_group_index: BlockGroupIndex(12),
        block_group_count: BlockGroupCount(5),
    }); // should panic
}
