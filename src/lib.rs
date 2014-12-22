#![feature(unboxed_closures, default_type_params)]
pub trait FnBox<Args, Result = ()> {
  extern "rust-call" fn call_box(self: Box<Self>, args: Args) -> Result;
}

#[cfg(when_coherence_is_fixed)]
impl<'a, Args, Result> FnOnce<Args, Result> for Box<FnBox<Args, Result> + 'a> {
  extern "rust-call" fn call_once(self, args: Args) -> Result {
    self.call_box(args)
  }
}

impl<F, Args, Result> FnBox<Args, Result> for F where F: FnOnce<Args, Result> {
  extern "rust-call" fn call_box(self: Box<F>, args: Args) -> Result {
    (*self).call_once(args)
  }
}

#[test]
fn can_be_boxed() {
  let f = move |:| {};
  let _: Box<FnBox<()>> = box f;
}

#[test]
fn can_be_called() {
  let mut called = false;

  {
    let f: Box<FnBox<()>> = box |:| { called = true };

    f();
  }

  assert_eq!(called, true);
}

#[test]
fn can_be_sent() {
  let f = move |:| {};
  let _: Box<FnBox<()> + Send> = box f;
}
