use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::address::{Address, TryAddress};
use super::device::Device;

#[derive(Debug, Default)]
pub struct Shared<T: ?Sized>(pub(crate) Inner<T>);

impl<T> Shared<T>
where
  T: 'static,
{
  pub fn new(dev: T) -> Self {
    Self(Rc::new(RefCell::new(dev)))
  }

  pub fn inner(&self) -> &Inner<T> {
    &self.0
  }

  pub fn inner_mut(&mut self) -> &mut Inner<T> {
    &mut self.0
  }
}

impl<T> Shared<T>
where
  T: ?Sized,
{
  pub fn borrow(&self) -> Ref<T> {
    self.0.borrow()
  }

  pub fn borrow_mut(&self) -> RefMut<T> {
    self.0.borrow_mut()
  }
}

impl<T> Address for Shared<T>
where
  T: Address + ?Sized,
{
  fn read(&self, addr: u16) -> u8 {
    self.0.read(addr)
  }

  fn write(&mut self, addr: u16, value: u8) {
    self.0.write(addr, value);
  }
}

impl<T> TryAddress for Shared<T>
where
  T: TryAddress + ?Sized,
{
  type Error = <T as TryAddress>::Error;

  fn try_read(&self, addr: u16) -> Result<u8, Self::Error> {
    self.0.try_read(addr)
  }

  fn try_write(&mut self, addr: u16, value: u8) -> Result<(), Self::Error> {
    self.0.try_write(addr, value)
  }
}

impl<T> Clone for Shared<T>
where
  T: ?Sized,
{
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> Device for Shared<T> where T: Device + ?Sized {}

impl<T> From<T> for Shared<T>
where
  T: 'static,
{
  fn from(dev: T) -> Self {
    Self::new(dev)
  }
}

impl<T> PartialEq for Shared<T>
where
  T: ?Sized,
{
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.0, &other.0)
  }
}

impl<T: ?Sized> Eq for Shared<T> {}

pub(crate) type Inner<T> = Rc<RefCell<T>>;

impl<T> Address for Inner<T>
where
  T: Address + ?Sized,
{
  fn read(&self, addr: u16) -> u8 {
    self.borrow().read(addr)
  }

  fn write(&mut self, addr: u16, value: u8) {
    self.borrow_mut().write(addr, value);
  }
}

impl<T> TryAddress for Inner<T>
where
  T: TryAddress + ?Sized,
{
  type Error = <T as TryAddress>::Error;

  fn try_read(&self, addr: u16) -> Result<u8, Self::Error> {
    self.borrow().try_read(addr)
  }

  fn try_write(&mut self, addr: u16, value: u8) -> Result<(), Self::Error> {
    self.borrow_mut().try_write(addr, value)
  }
}

impl<T> Device for Inner<T> where T: Device + ?Sized {}
