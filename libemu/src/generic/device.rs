use super::{address::Address, shared::Shared};

pub trait Device: Address {
  fn to_shared(self) -> Shared<Self>
  where
    Self: 'static + Sized,
  {
    self.into()
  }

  fn to_dynamic(self) -> Dynamic
  where
    Self: 'static + Sized,
  {
    self.to_shared().into()
  }
}

pub type Dynamic = Shared<dyn Device>;

impl<T> From<Shared<T>> for Dynamic
where
  T: Device + 'static,
{
  fn from(dev: Shared<T>) -> Self {
    Self(dev.0.clone())
  }
}
