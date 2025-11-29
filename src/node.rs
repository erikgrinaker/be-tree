use crate::page::PageId;

/// A key.
///
/// TODO: enforce a max size << PAGE_SIZE, e.g. 1 KB.
pub type Key = Vec<u8>;

/// A value.
///
/// TODO: enforce a max size << PAGE_SIZE, e.g. 1 KB.
/// TODO: split out large values into separate storage.
pub type Value = Vec<u8>;

/// A key range as [start, end).
pub type KeyRange = std::ops::Range<Key>;

/// An epoch, used for stable snapshots. This is a global logical timestamp or version number.
///
/// TODO: actually implement this.
pub type Epoch = u64;

/// A tree node.
#[derive(Clone, Debug)]
pub struct Node {
    /// The page ID serves both as a stable node identifier and an on-disk page identifier.
    pub id: PageId,
    /// TODO: implement CoW snapshots.
    /// TODO: integrate with MVCC somehow.
    pub epoch: Epoch,
    /// The key range covered by this node as [start, end).
    pub range: KeyRange,
    /// The node data.
    pub data: NodeData,
}

/// Data contained in a node.
///
/// TODO: consider separate root variant.
#[derive(Clone, Debug)]
pub enum NodeData {
    /// An internal node.
    Internal {
        /// Separator keys between child nodes.
        ///
        /// INVARIANT: len == children.len() - 1
        /// INVARIANT: sorted
        pivots: Vec<Key>,
        /// Child nodes.
        ///
        /// INVARIANT: child keys do not overlap.
        /// INVARIANT: children as sorted by key.
        children: Vec<(PageId, Epoch)>,
        /// Buffered mutations awaiting flush to children.
        buffer: Vec<Operation>,
    },
    /// A leaf node.
    Leaf { entries: Vec<(Key, Value)> },
}

/// A buffered operation in an internal node.
#[derive(Clone, Debug)]
pub enum Operation {
    /// Deletes a key.
    Delete { key: Key, epoch: Epoch },
    /// Sets a key to a value.
    Set { key: Key, value: Value, epoch: Epoch },
}
