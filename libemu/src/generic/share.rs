use std::{cell::RefCell, rc::Rc};

pub struct Shared<T: ?Sized>(Inner<T>);

type Inner<T> = Rc<RefCell<T>>;
