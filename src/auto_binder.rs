use crate::prelude::Bindable;
use std::fmt::Debug;
use std::vec::Vec;

#[derive(Clone, Default)]
pub struct AutoBinder<'a> {
    list: Vec<&'a dyn Bindable>,
}

impl<'a> AutoBinder<'a> {
    pub fn new(list: Vec<&'a dyn Bindable>) -> Self {
        for a in list.iter() {
            a.bind();
        }
        Self { list }
    }
}

impl<'a> Drop for AutoBinder<'a> {
    fn drop(&mut self) {
        self.list.reverse();
        for a in self.list.iter() {
            a.unbind();
        }
    }
}

/// 带槽位绑定器。
pub struct SlotBinder<'a> {
    bindable: &'a dyn Bindable,
    slot: u32,
}

impl<'a> Debug for SlotBinder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SlotBinder {{ bindable: {:p}, slot: {:?} }}",
            self.bindable, self.slot
        )
    }
}

impl<'a> Bindable for SlotBinder<'a> {
    fn bind(&self) {
        self.bind_at(self.slot);
    }

    fn bind_at(&self, slot: u32) {
        self.bindable.bind_at(slot);
    }

    fn unbind(&self) {
        self.bindable.unbind();
    }
}

impl<'a> SlotBinder<'a> {
    /// 创建一个新的带槽位绑定器。
    pub fn new(bindable: &'a dyn Bindable, slot: u32) -> Self {
        Self { bindable, slot }
    }
}
