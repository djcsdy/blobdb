# (Draft) Storage Layout

This document describes the low-level storage concepts in BlobDB. These concepts
are primarily relevant to BlobDB developers and implementers rather than
database users.

## Block

A fixed-size (4,096 bytes) on-disk container that holds between zero and 255
Packets. Any unused space within a Block is zero-padded. The storage of Blocks
depends on the Volume type:

* On block devices: Each Block maps directly to a physical device block.
* On filesystems: Blocks are stored sequentially within Files.

Block format:

8 bit packet count then either:

* One packet filling the whole block or
* Packet start index list written from beginning of block, packets themselves
  written in reverse order starting at end of block.

This split layout is intended to facilitate scanning a block to find a specific
Packet. The idea is that BlobDB only has to scan the first few bytes of a Block
to find the Packet it's looking for, ensuring cache locality. But this doesn't
quite work because metadata to identify the Packet is still in the Packet
itself, not at the beginning of the Block. So perhaps we need to split Packet
Metadata out from the Packet itself. This is tricky because at the moment our
design assumes Packet Metadata can't be standardized across all Packet types.

Fixed size of 4096 bytes facilitates data recovery from filesystem volumes, even
if the underlying filesystem is irretrievably corrupted.

But probably not necessary to have blocks that small for block device volumes.
Perhaps 16,384 bytes instead?

## Packet

The fundamental on-disk storage unit of BlobDB. A Packet consists of:

* A type ID that identifies the kind of data it contains.
* A sequence of zero or more bytes representing the actual data.

All persistent data in BlobDB is stored as Packets within Blocks. Different
Packet types are used to store different kinds of data (e.g., Blob data,
Allocation Tables, etc.).

## File

Files only exist for filesystem-based Volumes, where BlobDB data is organized in
a hierarchy of standard files and directories. Each file in this hierarchy
contains a sequence of Blocks. This concept does not apply to block device
Volumes.

## Allocation Table

A tracking system for block device Volumes that maintains the usage status
(free or in-use) of all blocks. The Allocation Table:

* Is consulted when new Blocks need to be allocated.
* Is updated when Blocks are freed.
* Is itself stored as a series of Packets within Blocks.
* Exists only for block device Volumes.

## Change

An atomic operation that alters the Database state. Common types of Changes
include:

* Creating a new Index.
* Deleting an Index.
* Creating a Key-Value Mapping.
* Deleting a Key-Value Mapping.

All modifications to a Database must be performed through Changes to ensure
atomicity.

## Journal

An ordered log of Changes that provides durability and consistency for the
Database. Key characteristics:

* Every Database has one Journal.
* Changes become effective once fully written to the Journal.
* Changes are gradually removed from the Journal's beginning as they are
  applied to on-disk data structures.
* The Journal is the source of truth:
    * It overrides contradictions with other on-disk structures.
    * For conflicting Changes, the later Journal entry prevails.
    * Partially written Changes are considered invalid.
* Like all BlobDB data, the Journal is stored as Packets within Blocks.