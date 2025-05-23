use crate::block_device::extent::Extent;

#[test]
fn end_block_index() {
    let extent = Extent {
        first_block_index: 5,
        block_count: 3,
    };
    assert_eq!(extent.end_block_index(), 8);

    let extent = Extent {
        first_block_index: 0,
        block_count: 1,
    };
    assert_eq!(extent.end_block_index(), 1);
}

#[test]
fn split() {
    // Test normal split
    let extent = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    let (first, second) = extent.split(3);
    assert_eq!(
        first,
        Extent {
            first_block_index: 10,
            block_count: 3,
        }
    );
    assert_eq!(
        second,
        Some(Extent {
            first_block_index: 13,
            block_count: 2,
        })
    );

    // Test split at exact size
    let extent = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    let (first, second) = extent.split(5);
    assert_eq!(first, extent);
    assert_eq!(second, None);
}

#[test]
#[should_panic(expected = "block_count must be less than or equal to extent block_count")]
fn split_panic() {
    let extent = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    extent.split(6); // Should panic
}

#[test]
fn merge() {
    // Test successful merge
    let extent1 = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    let extent2 = Extent {
        first_block_index: 15,
        block_count: 3,
    };
    assert_eq!(
        extent1.merge(extent2),
        Some(Extent {
            first_block_index: 10,
            block_count: 8,
        })
    );

    // Test merge with gap
    let extent1 = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    let extent2 = Extent {
        first_block_index: 16,
        block_count: 3,
    };
    assert_eq!(extent1.merge(extent2), None);

    // Test merge with reversed order
    let extent1 = Extent {
        first_block_index: 15,
        block_count: 3,
    };
    let extent2 = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    assert_eq!(
        extent1.merge(extent2),
        Some(Extent {
            first_block_index: 10,
            block_count: 8,
        })
    );
}

#[test]
fn overlaps() {
    // Test overlapping extents
    let extent1 = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    let extent2 = Extent {
        first_block_index: 13,
        block_count: 3,
    };
    assert!(extent1.overlaps(extent2));
    assert!(extent2.overlaps(extent1));

    // Test adjacent but non-overlapping extents
    let extent1 = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    let extent2 = Extent {
        first_block_index: 15,
        block_count: 3,
    };
    assert!(!extent1.overlaps(extent2));
    assert!(!extent2.overlaps(extent1));

    // Test completely separate extents
    let extent1 = Extent {
        first_block_index: 10,
        block_count: 5,
    };
    let extent2 = Extent {
        first_block_index: 20,
        block_count: 3,
    };
    assert!(!extent1.overlaps(extent2));
    assert!(!extent2.overlaps(extent1));

    // Test one extent completely containing another
    let extent1 = Extent {
        first_block_index: 10,
        block_count: 10,
    };
    let extent2 = Extent {
        first_block_index: 12,
        block_count: 3,
    };
    assert!(extent1.overlaps(extent2));
    assert!(extent2.overlaps(extent1));
}
