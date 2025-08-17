# Bε-Tree

An aspirational [Bε-tree](https://www.usenix.org/system/files/login/articles/login_oct15_05_bender.pdf)
implementation in Rust. To be used as a storage engine with Accord distributed consensus.

## Design Ideas

* Copy-on-write support.
    * Cheap atomic snapshots.
    * No need for a WAL.
    * Logical page table to avoid write amp.
    * See e.g. FoundationDB Redwood.
* MVCC support.
    * Integrate MVCC with CoW.
    * Garbage collection during node flushes.
* WiscKey-like value separation (for >1KB values).
* Range tombstones.