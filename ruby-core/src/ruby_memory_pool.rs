use crate::RubyState;

/**
Represents Ruby's memory pool

Corresponds to mruby's `mrb_pool` struct.
*/
pub struct RubyMemoryPool<'a> {
    ruby_state: &'a RubyState,
    pages: Vec<RubyMemoryPoolPage>
}

/**
Represents a page within Ruby's memory pool

Corresponds to mruby's `mrb_pool_page` struct.
*/
pub struct RubyMemoryPoolPage {
    pub page: Vec<u8>
}

impl<'a> RubyMemoryPool<'a> {
    pub fn new(ruby_state: &'a RubyState) -> Self {
        Self {
            ruby_state: ruby_state,
            pages: Vec::new()
        }
    }
    pub fn alloc(&mut self, capacity: usize) -> &RubyMemoryPoolPage {
        let page = RubyMemoryPoolPage {
            page: Vec::with_capacity(capacity)
        };
        self.pages.push(page);
        self.pages.last().unwrap()
    }
}
