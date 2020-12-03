/// Set object state in current context.
pub trait Bindable {
    /// Bind to current context.
    fn bind(&self) {}
    /// Bind to current context with slot.
    fn bind_at(&self, _slot: u32) {}
    /// Release from current context.
    fn unbind(&self) {}
    /// Release from current context with slot.
    fn unbind_at(&self, _slot: u32) {}
}
