//! Pinning implementation for [`[T]`] with [`role::Items`].
//!
//! The added [`ItemsPin<role::Items, [T]>`](`self`) methods are:
//!
//! - TODO

use crate::{role, Items, ItemsMut, ItemsPin};
use core::slice::{Iter, IterMut};
use std::{mem::ManuallyDrop, ops::Range};

impl<'a, T: 'a> Items<'a, role::Items> for [T] {
	type Item = T;

	type ItemsIter = Iter<'a, T>;

	fn items(&'a self) -> Self::ItemsIter {
		self.iter()
	}
}

impl<'a, T: 'a> ItemsMut<'a, role::Items> for [T] {
	type ItemsMutIter = IterMut<'a, T>;

	fn items_mut(&'a mut self) -> Self::ItemsMutIter {
		self.iter_mut()
	}
}

impl<T> ItemsPin<role::Items, [T]> {
	#[must_use]
	pub fn pin(slice: &'static mut [T]) -> &'static mut Self {
		unsafe { &mut *(slice as *mut _ as *mut _) }
	}

	/// Drops all items outside `range` in place and returns the remaining subslice.
	///
	/// # Panics
	///
	/// Iff `range.start > range.end || range.end > self.len()`.
	pub fn truncate(&'static mut self, range: Range<usize>) -> &'static mut Self {
		if range.start > range.end || range.end > self.len() {
			panic!("Invalid range.")
		}

		let (a, bc) = self.collection.split_at_mut(range.start);
		let (b, c) = bc.split_at_mut(range.len());
		Self::pin(a).drop_in_place();
		Self::pin(c).drop_in_place();
		Self::pin(b)
	}

	pub fn drop_in_place(&'static mut self) {
		unsafe {
			for value in &mut *(self as *mut _ as *mut [ManuallyDrop<T>]) {
				ManuallyDrop::drop(value)
			}
		}
	}
}
