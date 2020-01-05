use crate::Ruby;

/// Represents Ruby's memory pool
#[maps_to(mruby: mrb_pool)]
pub struct MemoryPool<'a> {
    rb: &'a Ruby,
    pages: Vec<MemoryPoolPage>,
}

/// Represents a page within Ruby's memory pool
#[maps_to(mruby: mrb_pool_page)]
pub struct MemoryPoolPage {
    pub page: Vec<u8>,
}

impl<'a> MemoryPool<'a> {
    /// Creates a new memory pool
    #[maps_to(mruby: mrb_pool_open(mrb_state *mrb))]
    pub fn new(rb: &'a Ruby) -> Self {
        Self {
            rb: rb,
            pages: Vec::new(),
        }
    }
    /// Allocates a new page within a memory pool
    #[maps_to(mruby: mrb_pool_alloc(mrb_pool *pool, size_t len))]
    pub fn alloc(&mut self, capacity: usize) -> &MemoryPoolPage {
        let page = MemoryPoolPage {
            page: Vec::with_capacity(capacity),
        };
        self.pages.push(page);
        self.pages.last().unwrap()
    }
}
