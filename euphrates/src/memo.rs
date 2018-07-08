//! Memos - simple messages sent from devices to the user
//!
//! Memos are useful for debugging.

use std::fmt::Display;
use std::marker::PhantomData;

use impler::Impl;

pub trait Inbox {
    type Memo;

    fn receive_impl(&mut self, memo: Self::Memo);

    #[inline]
    fn receive(&mut self, memo: Self::Memo) {
        if self.active() {
            self.receive_impl(memo);
        }
    }

    #[inline]
    fn active(&self) -> bool {
        true
    }
}

pub struct InboxImpl;

impl<T> Inbox for T
where
    T: Impl<InboxImpl> + ?Sized,
    T::Impler: Inbox,
{
    type Memo = <<T as Impl<InboxImpl>>::Impler as Inbox>::Memo;

    #[inline]
    fn receive_impl(&mut self, memo: Self::Memo) {
        self.make_mut().receive_impl(memo)
    }

    #[inline]
    fn receive(&mut self, memo: Self::Memo) {
        self.make_mut().receive(memo)
    }

    #[inline]
    fn active(&self) -> bool {
        self.make().active()
    }
}

/// An Inbox that throws away its memos.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct NothingInbox<M>(PhantomData<M>);

impl<M> Default for NothingInbox<M> {
    #[inline]
    fn default() -> Self {
        NothingInbox(PhantomData)
    }
}

impl<M> Inbox for NothingInbox<M> {
    type Memo = M;
    #[inline]
    fn receive_impl(&mut self, _memo: M) {}
    #[inline]
    fn active(&self) -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct PrintingInbox<M: ?Sized>(PhantomData<M>);

impl<M> Default for PrintingInbox<M> {
    #[inline]
    fn default() -> Self {
        PrintingInbox(PhantomData)
    }
}

impl<M> Inbox for PrintingInbox<M>
where
    M: Display,
{
    type Memo = M;

    #[inline]
    fn receive_impl(&mut self, memo: M) {
        println!("{}", memo);
    }
}