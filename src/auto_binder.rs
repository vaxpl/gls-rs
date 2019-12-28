use crate::prelude::Bindable;
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
