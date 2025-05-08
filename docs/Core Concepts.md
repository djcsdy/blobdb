# Core Concepts

* **Blob**: An immutable sequence of zero or more bytes. Blobs serve as both
  Keys and Values in BlobDB, and are stored at the Database level. A Blob
  persists in the Database only as long as it is referenced by at least one
  Key-Value mapping in any Index.

* **Ordering**: A set of rules that determine how Keys are lexicographically
  ordered. Orderings can be user-defined, or one of the built-in orderings:
 
    * **Bytewise**: Lexicographic ordering by byte values.

    * **UTF-8**: Lexicographic ordering by Unicode codepoints, treating the
      Blob as a UTF-8 encoded string.

* **Database**: A collection of Blobs along with zero or more Indexes that
  maintain ordered Key-Value mappings between these Blobs. Blobs are automatically
  removed from the Database when they are no longer referenced by any Index.

* **Index**: An ordered collection of Key-Value mappings with an associated
  Ordering, where:

    * Keys and Values are references to Blobs stored in the Database.

    * Each Key maps to exactly one Value.

    * Keys are sorted and compared according to the Index's Ordering.

    * Each Index operates independently with no consistency guarantees relative
      to other Indexes.

* **Volume**: A storage medium that can host zero or more Databases. A Volume
  can be either:
 
    * A directory in a POSIX filesystem containing a BlobDB file hierarchy.

    * A raw block device formatted for BlobDB.