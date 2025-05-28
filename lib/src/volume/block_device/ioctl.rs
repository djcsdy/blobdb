use linux_raw_sys::ioctl::BLKGETSIZE64;
use rustix::io::Result;
use rustix::ioctl;
use std::os::fd::AsFd;

/// `ioctl(fd, BLKGETSIZE64)`—Returns the logical block size of a block device.
pub fn ioctl_blkgetsize<Fd: AsFd>(fd: Fd) -> Result<u64> {
    // SAFETY: BLKGETSIZE64 is a getter opcode that gets a `u64`.
    unsafe { ioctl::ioctl(fd, ioctl::Getter::<{ BLKGETSIZE64 }, u64>::new()) }
}
