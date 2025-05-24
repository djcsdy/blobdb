# (Draft) Power Loss Protection

## System Requirements

Block Device Volumes require:
* Linux kernel 5.1 or later (for io_uring support)
* Raw block devices that preserve data through power loss
* Direct I/O support
* Appropriate permissions to access block devices

## Write Safety Strategy

BlobDB employs a multi-layered approach to ensure data durability on block devices:

### 1. Block-Level Integrity
* All data is written in 4KB block units
* SHA-256 digest protects each block
* Invalid digests indicate interrupted writes
* No block containing valid data is ever modified in place
* Empty or corrupted blocks are safely ignored
* Atomic semantics achieved without requiring atomic hardware writes

### 2. Direct Block Device I/O
* Uses io_uring with `O_DIRECT` flag
* Bypasses kernel page cache entirely
* Prevents cached writes that could be lost
* Direct hardware access for predictable behavior

### 3. Write Barriers
* Uses io_uring barrier operations
* Ensures writes reach persistent storage
* Prevents reordering of critical writes
* Applied after journal updates

### 4. Asynchronous Completion
* io_uring completion queue monitors writes
* Hardware notifications when data is persistent
* Enables efficient write batching
* Maintains high throughput

## Implementation Details

### Write Path
1. Data is formatted into 4KB blocks with SHA-256 digest
2. Blocks are written to empty locations only
3. Journal records are written first
4. Data blocks are written with barriers
5. Completion is confirmed via io_uring
6. Journal is updated to mark completion

### Recovery Process
1. Scan all blocks for valid digests
   * Any block with an invalid digest is treated as empty
   * Partial writes are automatically detected and ignored
2. Load journal contents
3. Identify incomplete operations
4. Roll back partial transactions
5. Restore consistent state

## Performance Considerations

### Optimizations
* Batched writes through io_uring
* Efficient barrier placement
* Zero-copy I/O where possible
* Completion notification batching

### Trade-offs
* Direct I/O requires aligned buffers
* Write barriers add some latency
* Recovery scan time scales with volume size
* Memory requirements for write batching

## Testing

Block Device Volume power loss protection should be verified through:
1. Simulated power failures
2. Random write interruption
3. System crash testing
4. Corruption detection checks
5. Recovery process validation

## Monitoring

The following should be monitored:
* Write completion latency
* Barrier operation timing
* Recovery scan duration
* Invalid block detection
* Journal replay events