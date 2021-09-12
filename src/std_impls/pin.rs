//! Pinning implementation for [`Pin<C>`]
//! where [`C: Items<R>`](`Items`) or [`C: ItemsMut<R>`](`ItemsMut`)
//! where `R` is **any** [`Role`].
//!
//! This doesn't add any special methods
//! (and you'll have to implement pinning and unpinning yourself),
//! but it gives you some auxiliary implementation to work with.

use crate::{role::Role, Items, ItemsMut, ItemsPinned, ItemsPinnedMut, PinIter};
use core::pin::Pin;

impl<'a, R: Role, C> Items<'a, R> for Pin<C>
where
	C: Items<'a, R>,
{
	type Item = C::Item;
	type ItemsIter = C::ItemsIter;

	fn items(&'a self) -> Self::ItemsIter {
		unsafe { &*(self as *const Pin<C>).cast::<C>() }.items()
	}
}
impl<'a, R: Role, C> ItemsPinned<'a, R> for Pin<C>
where
	C: Items<'a, R>,
{
	type ItemsPinnedIter = PinIter<C::ItemsIter>;

	fn items_pinned(&'a self) -> Self::ItemsPinnedIter {
		unsafe { PinIter::new_unchecked(self.items()) }
	}
}

impl<'a, R: Role, C> ItemsMut<'a, R> for Pin<C>
where
	C: ItemsMut<'a, R>,
{
	type ItemsMutIter = C::ItemsMutIter;

	fn items_mut(&'a mut self) -> Self::ItemsMutIter {
		unsafe { &mut *(self as *mut Pin<C>).cast::<C>() }.items_mut()
	}
}
impl<'a, R: Role, C> ItemsPinnedMut<'a, R> for Pin<C>
where
	C: ItemsMut<'a, R>,
{
	type ItemsPinnedMutIter = PinIter<C::ItemsMutIter>;

	fn items_pinned_mut(&'a mut self) -> Self::ItemsPinnedMutIter {
		unsafe { PinIter::new_unchecked(self.items_mut()) }
	}
}
