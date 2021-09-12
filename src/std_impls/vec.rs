//! Pinning implementation for [`Vec<T>`] with [`role::Items`].
//!
//! The added [`ItemsPin<role::Items, Vec<T>>`](`self`) methods are:
//!
//! - `.as_slice`, which narrows [`Vec::as_slice`] to return [`&ItemsPin<role::Items; [T]>`](`super::slice`).
//! - `.as_slice_mut`, which narrows [`Vec::as_slice_mut`] to return [`&mut ItemsPin<role::Items; [T]>`](`super::slice`).
//! - `.leak`, which narrows [`Vec::leak`] to return [`&'static mut ItemsPin<role::Items; [T]>`](`super::slice`).
//! - `.pop_pinned`, which drops the last value in place if possible, returning [`bool`].
//! - `.push_pinned`, which allows limited insertions even after pinning.
//! - `.truncate_pinned`, which forwards [`Vec::truncate`].

use crate::{role, Items, ItemsMut, ItemsPin};
use std::slice;

impl<'a, T: 'a> Items<'a, role::Items> for Vec<T> {
	type Item = T;

	type ItemsIter = slice::Iter<'a, T>;

	fn items(&'a self) -> Self::ItemsIter {
		self.iter()
	}
}

impl<'a, T: 'a> ItemsMut<'a, role::Items> for Vec<T> {
	type ItemsMutIter = slice::IterMut<'a, T>;

	fn items_mut(&'a mut self) -> Self::ItemsMutIter {
		self.iter_mut()
	}
}

impl<T> ItemsPin<role::Items, Vec<T>> {
	#[must_use]
	pub fn pin(vec: Vec<T>) -> Self {
		unsafe { ItemsPin::new_unchecked(vec) }
	}

	/// # Errors
	///
	/// Iff the underlying [`Vec`] does not have any spare capacity.
	pub fn push_pinned(&mut self, value: T) -> Result<(), T> {
		if self.collection.len() < self.collection.capacity() {
			self.collection.push(value);
			Ok(())
		} else {
			Err(value)
		}
	}

	pub fn pop_pinned(&mut self) -> bool {
		!self.collection.is_empty() && {
			self.collection.truncate(self.collection.len() - 1);
			true
		}
	}

	pub fn truncate_pinned(&mut self, len: usize) {
		self.collection.truncate(len)
	}

	#[must_use]
	pub fn as_slice(&self) -> &ItemsPin<role::Items, [T]> {
		unsafe { &*(self.collection.as_slice() as *const _ as *const _) }
	}

	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut ItemsPin<role::Items, [T]> {
		unsafe { &mut *(self.collection.as_mut_slice() as *mut _ as *mut _) }
	}

	#[must_use]
	pub fn leak(self) -> &'static mut ItemsPin<role::Items, [T]> {
		unsafe { &mut *(self.collection.leak() as *mut _ as *mut _) }
	}
}
