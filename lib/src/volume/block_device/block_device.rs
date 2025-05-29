use crate::block::Block;
use crate::volume::block_device::allocation_tree::AllocationTree;
use crate::volume::block_device::block_group_count::BlockGroupCount;
use crate::volume::block_device::byte_count::ByteCount;
use crate::volume::block_device::ioctl;
use linux_raw_sys::general::{S_IFBLK, S_IFMT};
use rustix::fs;
use rustix::fs::{Mode, OFlags};
use rustix::io::Result;
use rustix::path;

#[derive(Debug)]
pub struct BlockDevice {
    physical_block_size: ByteCount<u32>,
    block_group_size: u32,
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

        let physical_block_size = ByteCount(fs::ioctl_blkpbszget(&fd)?);
        if !Self::is_valid_physical_block_size(physical_block_size) {
            return Err(rustix::io::Errno::INVAL);
        }

        let block_group_size = if physical_block_size < ByteCount(4096) {
            1
        } else {
            *physical_block_size / 4096
        };

        let block_group_count = BlockGroupCount(
            ioctl::ioctl_blkgetsize(&fd)? / (Block::SIZE as u64 * block_group_size as u64),
        );
        let allocation_tree = AllocationTree::new(block_group_count);

        // TODO

        Ok(Self {
            physical_block_size,
            block_group_size,
            allocation_tree,
        })
    }

    fn is_valid_physical_block_size(block_size: ByteCount<u32>) -> bool {
        block_size >= ByteCount(512) && block_size.is_power_of_two()
    }
}
