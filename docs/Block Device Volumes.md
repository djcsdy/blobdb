# (Draft) Block Device Volumes

A Block Device Volume is a storage Volume that operates directly on a raw block device.

## Block Device Requirements

* The block device must preserve data through power loss

## Volume Structure

A Block Device Volume is formatted into a sequence of 4KB blocks. Each block serves one of the following purposes:

* **Volume Header Block**: Contains Volume metadata and configuration
* **Database Allocation Blocks**: Track which blocks are allocated to which Databases
* **Database Header Blocks**: Contain Database metadata and configuration
* **Data Blocks**: Store the actual Database content (Blobs and Indexes)
* **Journal Blocks**: Record ongoing Database modifications
* **Free Blocks**: Available for allocation

### Volume Header Block

The Volume Header Block is always the first block (offset 0) and contains:

* Magic number to identify BlobDB Volume format
* Volume format version
* Block size (always 4KB)
* Total number of blocks in Volume
* Location and size of allocation tables
* Volume creation timestamp
* Volume UUID

### Database Allocation

The Volume maintains allocation tables to track block ownership:

* Each Database is assigned a unique 16-byte Database ID
* Allocation tables map block ranges to Database IDs
* Free blocks are marked with a special null Database ID
* Allocation changes are atomic and logged in the journal

## Block Device Access

BlobDB accesses the block device through direct I/O to ensure:

* Writes are atomic at the 4KB block level
* No operating system page cache interference
* Immediate durability of changes
* Consistent performance

### Access Patterns

* **Sequential Writes**: Journal updates and new data blocks
* **Random Reads**: Index lookups and Blob access
* **Random Writes**: In-place updates (only for allocation tables)

## Durability and Recovery

Block Device Volumes maintain data integrity through:

* Atomic 4KB block writes
* Block-level checksums (SHA-256)
* Write-ahead journaling
* Power failure recovery procedures

### Recovery Process

1. Read Volume Header Block
2. Verify block checksums
3. Scan journal blocks
4. Apply or roll back incomplete transactions
5. Verify allocation tables
6. Mount Databases

## Performance Considerations

* Direct I/O bypasses the operating system page cache
* 4KB block size matches common storage device page sizes
* Sequential journal writes optimize for modern storage devices
* Allocation tables support efficient block reuse
* Multiple Databases can share a Volume efficiently

## Security

* Block Device Volumes require appropriate permissions
* Raw block devices should be access-controlled
* Each Database's blocks are isolated by Database ID
* Block checksums prevent data corruption

## Limitations

* Minimum Volume size: 16MB (4096 blocks)
* Maximum Volume size: 16TB (4 billion blocks)
* Maximum Databases per Volume: 2^64 (Database ID space)
* Block size fixed at 4KB