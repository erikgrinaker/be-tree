# Bε-Tree

An aspirational [Bε-tree](https://www.usenix.org/system/files/login/articles/login_oct15_05_bender.pdf)
implementation in Rust. To be used as a storage engine with Accord distributed consensus.

## Design Ideas

* Copy-on-write pages.
    * Cheap atomic snapshots.
    * Effectively latch-free for reads.
    * No need for a WAL.
    * Page buffers amortize CoW write amp.
    * Logical page table to avoid write amp (pointer update ripples to root).
    * See e.g. FoundationDB Redwood.
* MVCC support.
    * Integrate MVCC with CoW (same timestamps).
    * Garbage collection during node flushes.
* WiscKey-like value separation (for >1KB values).
* Range tombstones.