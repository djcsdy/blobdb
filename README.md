# BlobDB

BlobDB is a key-value database that stores arbitrarily large binary objects
(blobs).


## Project Status

BlobDB is a work in progress, and under active development. The current
implementation is experimental and does not yet match the documentation.


## Principles

 * **Optimized for Immutable Blobs**: Blobs can be added and deleted, but are
   otherwise expected to remain unchanged after creation.
 * **Distributed Storage**: A single database can span multiple storage media,
   and a single storage medium can host many databases.
 * **Highly recoverable**: Blobs can be stored with configurable levels of
   redundancy. All data is stored in a format that maximizes recoverability in
   the event of data corruption or media failure.
 * **Storage Medium Agnostic**: Databases can be stored in a directory within a
   standard POSIX filesystem, or directly on raw block devices.
 * **Orthogonal Design**, **No Artificial Size Limitations**: Both keys and
   values are arbitrarily large blobs.
 * **Flexible Key Ordering**: Keys can be sorted and iterated according to a
   built-in or user-defined lexicographic ordering.

   
## Use Cases

BlobDB is intended for usage scenarios such as:

 * Archiving large files or large static datasets.
 * Backups.
 * Write-once, read-many (WORM) storage scenarios.
