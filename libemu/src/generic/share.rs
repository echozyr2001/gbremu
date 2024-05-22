use std::{
  cell::{Ref, RefCell, RefMut},
  rc::Rc,
};

use super::{
  arch::{
    address::{Address, TryAddress},
    value::Value,
  },
  device::Device,
  pcb::Board,
  reg::Cell,
};

/// Heap-allocated multi-access reference.
#[derive(Debug, Default)]
pub struct Shared<T: ?Sized>(pub Inner<T>);
/// Internal shared reference type.
pub type Inner<T> = Rc<RefCell<T>>;

impl<T> Shared<T>
where
  T: 'static,
{
  /// Constructs a new [`Shared`] resource.
  pub fn new(dev: T) -> Self {
    Self(Rc::new(RefCell::new(dev)))
  }

  // /// Gets a reference to the inner
  // pub fn inner(&self) -> &Inner<T> {
  //   &self.0
  // }

  // /// Gets a mutable reference to the inner
  // pub fn inner_mut(&mut self) -> &mut Inner<T> {
  //   &mut self.0
  // }
}

impl<T> Clone for Shared<T>
where
  T: ?Sized,
{
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Shared<T>
where
  T: ?Sized,
{
  /// Borrows the inner resource.
  pub fn borrow(&self) -> Ref<T> {
    self.0.borrow()
  }

  /// Borrows the inner resource mutably.
  pub fn borrow_mut(&self) -> RefMut<T> {
    self.0.borrow_mut()
  }
}

impl<Idx, V, T> Board<Idx, V> for Shared<T>
where
  T: Board<Idx, V> + ?Sized,
  Idx: Value,
  V: Value,
{
  fn connect(&self, bus: &mut super::bus::Bus<Idx, V>) {
    self.0.connect(bus)
  }
}

impl<T> From<T> for Shared<T>
where
  T: 'static,
{
  fn from(dev: T) -> Self {
    Self::new(dev)
  }
}

impl<T, Idx, V> Device<Idx, V> for Shared<T>
where
  T: Device<Idx, V> + ?Sized,
  Idx: Value,
  V: Value,
{
}

impl<T, Idx, V> Address<Idx, V> for Shared<T>
where
  T: Address<Idx, V> + ?Sized,
  Idx: Value,
  V: Value,
{
  fn read(&self, idx: Idx) -> V {
    self.0.read(idx)
  }

  fn write(&mut self, idx: Idx, val: V) {
    self.0.write(idx, val)
  }
}

impl<T, Idx, V> Address<Idx, V> for Inner<T>
where
  T: Address<Idx, V> + ?Sized,
  Idx: Value,
  V: Value,
{
  fn read(&self, index: Idx) -> V {
    self.borrow().read(index)
  }

  fn write(&mut self, index: Idx, value: V) {
    self.borrow_mut().write(index, value);
  }
}

impl<T, Idx, V> Board<Idx, V> for Inner<T>
where
  T: Board<Idx, V> + ?Sized,
  Idx: Value,
  V: Value,
{
  fn connect(&self, bus: &mut super::bus::Bus<Idx, V>) {
    self.borrow().connect(bus);
  }
}

impl<V, T> Cell<V> for Inner<T>
where
  T: Cell<V> + ?Sized,
  V: Value,
{
  fn load(&self) -> V {
    self.borrow().load()
  }

  fn store(&mut self, value: V) {
    self.borrow_mut().store(value)
  }
}

impl<V, T> Cell<V> for Shared<T>
where
  T: Cell<V> + ?Sized,
  V: Value,
{
  fn load(&self) -> V {
    self.0.load()
  }

  fn store(&mut self, value: V) {
    self.0.store(value)
  }
}

// impl<T, Idx, V> TryAddress<Idx, V> for Shared<T>
// where
//   T: TryAddress<Idx, V> + ?Sized,
//   Idx: Value,
//   V: Value,
// {
//   type Error = <T as TryAddress<Idx, V>>::Error;

//   fn try_read(&self, idx: Idx) -> Result<V, Self::Error> {
//     self.borrow().try_read(idx)
//   }

//   fn try_write(&mut self, idx: Idx, val: V) -> Result<(), Self::Error> {
//     self.borrow_mut().try_write(idx, val)
//   }
// }
