use crate::block::Block;
use crate::units::{BlockCount, BlockGroupCount, ByteCount};
use crate::volume::block_device::allocation_tree::AllocationTree;
use crate::volume::block_device::ioctl;
use linux_raw_sys::general::{S_IFBLK, S_IFMT};
use rustix::fs;
use rustix::fs::{Mode, OFlags};
use rustix::io::Result;
use rustix::path;

#[derive(Debug)]
pub struct BlockDevice {
    physical_block_size: ByteCount,
    block_group_size: BlockCount,
    allocation_tree: AllocationTree,
}

impl BlockDevice {
    pub fn format(path: impl path::Arg) -> Result<Self> {
        let fd = fs::open(
            path,
            OFlags::RDWR | OFlags::DIRECT | OFlags::EXCL | OFlags::SYNC,
            Mode::empty(),
        )?;

        if !fs::fstat(&fd)
            .map(|stat| (stat.st_mode & S_IFMT) == S_IFBLK)
            .unwrap_or(false)
        {
            return Err(rustix::io::Errno::NOTBLK);
        }

        let physical_block_size = ByteCount(fs::ioctl_blkpbszget(&fd)? as u64);
        if !physical_block_size.is_power_of_two() {
            return Err(rustix::io::Errno::INVAL);
        }

        let block_group_size = physical_block_size.to_block_count().max(BlockCount(1));

        let block_group_count = BlockGroupCount(
            ioctl::ioctl_blkgetsize(&fd)? / (*Block::SIZE * (*block_group_size as u64)),
        );
        let allocation_tree = AllocationTree::new(block_group_count);

        // TODO

        Ok(Self {
            physical_block_size,
            block_group_size,
            allocation_tree,
        })
    }
}
