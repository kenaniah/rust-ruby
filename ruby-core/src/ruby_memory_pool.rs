use crate::RubyState;

/// Represents Ruby's memory pool
///
/// Corresponds to mruby's `mrb_pool` struct.
pub struct RubyMemoryPool<'a> {
    ruby_state: &'a RubyState,
    pages: Vec<RubyMemoryPoolPage>,
}

/// Represents a page within Ruby's memory pool
///
/// Corresponds to mruby's `mrb_pool_page` struct.
pub struct RubyMemoryPoolPage {
    pub page: Vec<u8>,
}

impl<'a> RubyMemoryPool<'a> {
    /// Creates a new memory pool
    ///
    /// Corresponds to mruby's `mrb_pool_open(mrb_state *mrb)`
    pub fn new(ruby_state: &'a RubyState) -> Self {
        Self {
            ruby_state: ruby_state,
            pages: Vec::new(),
        }
    }
    /// Allocates a new page within a memory pool
    ///
    /// Corresponds to mruby's `mrb_pool_alloc(mrb_pool *pool, size_t len)`
    pub fn alloc(&mut self, capacity: usize) -> &RubyMemoryPoolPage {
        let page = RubyMemoryPoolPage {
            page: Vec::with_capacity(capacity),
        };
        self.pages.push(page);
        self.pages.last().unwrap()
    }
}
