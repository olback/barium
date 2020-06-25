use std::cell::RefCell;

pub fn clone_inner<T: Clone>(refcell: &RefCell<T>) -> T {

    (*refcell.borrow()).clone()

}
