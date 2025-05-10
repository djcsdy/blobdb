# Core Concepts

## Blob

An immutable sequence of zero or more bytes.

Blobs are stored within a Database. A Blob persists only as long as it is
referenced by at least one Key-Value Mapping in any Index.

## Key

A Blob that serves as an identifier in a Key-Value Mapping.

Within an Index, each Key must be unique, but the same Key may be present in
any number of Indexes. Within an Index, Keys are sorted and compared using the
Ordering configured for that Index.

## Value

A Blob that stores data in a Key-Value Mapping.

Unlike Keys, Values have no inherent ordering. The same Value may be mapped by
any number of Keys in any number of Indexes.

## Ordering

A rule that defines a lexicographic ordering of Keys.

BlobDB provides the following built-in Orderings:

* **Bytewise**: Keys are ordered lexicographically based on byte values.
* **UTF-8**: Keys are treated as UTF-8 strings and ordered lexicographically
  based on Unicode code points.

The user can also provide their own implementations of custom Orderings.

## Key-Value Mapping

Defines a many-to-one relationship within an Index where a Key maps to a
specified Value.

A Key-Value Mapping relates the Key and Value by reference. The Key and Value
are both Blobs stored within the same Database as the Index.

## Index

An ordered collection of Key-Value Mappings within a Database.

Each Index has an associated Ordering. Within the Index, Key-Value Mappings are
stored in the order of their Keys as defined by the associated Ordering.

The structure of an Index makes it possible to efficiently look up a Value
given the Key that maps to that value. It is also possible to efficiently
iterate through the Key-Value mappings in forward or reverse order as defined by
the associated Ordering.

Within an Index, each Key must be unique, but the same Value may be mapped by
any number of Keys.

Indexes are independent of one another. There is no automatic guarantee of
consistency between Indexes.

## Database

A logical container of Blobs and Indexes, hosted on one or more Volumes.

A Blob persists within a Database only as long as it is referenced by at least
one Key-Value mapping in any Index. Each Blob is stored once within the
Database even if it is referenced multiple times.

A Database provides atomicity and consistency guarantees. These guarantees are
implemented and enforced by a journaling system within the Database.

Databases are independent of one another. There is no automatic guarantee of
consistency between Databases.

The user specifies which Volumes will host a Database when they create the
Database. The user may also reconfigure which Volumes host a running Database,
capacity permitting.

Hosting a Database on more than one Volume can be used to increase capacity and
performance and also provides additional durability options.

## Volume

A physical or logical storage medium that can host any number of Databases.

A Volume is the foundational storage layer of BlobDB. It abstracts the
underlying storage mechanism, which can be either:

* **Filesystem**: A directory hierarchy within a POSIX filesystem used to store
  BlobDB data in an opaque binary format.
* **Block Device**: A raw block device formatted for BlobDB use.

Each Volume may host multiple independent Databases. A single Database may also
divide its storage across multiple Volumes.

Volumes are independent of one another. There is no automatic guarantee of
consistency between Volumes except as provided by the Databases hosted by those
Volumes.