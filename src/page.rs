pub const PAGE_SIZE: usize = 16 * 1024;

pub type PageId = u64;

pub struct Page {
    pub data: [u8; PAGE_SIZE],
}
