#![doc(html_root_url = "https://docs.rs/napje/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::inline_always)] // `const fn` with added constraints is unstable.

use std::{ops::Deref, pin::Pin};

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

/// A pinning wrapper for a collection type `C` that can pin-project to its items while pinned this way.
///
/// Unlike when using [`Pin<&C>`](`std::pin::Pin`), this allows the collection itself to stay [`Unpin`].
///
/// [`ItemsPin<C>`] acts to [`C: Items<Item = T>`](`Items`) as [`Pin<P>`](`std::pin::Pin`) does to [`P: Deref<Target = T>`](`std::ops::Deref`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemsPin<C: ?Sized> {
	collection: C,
}

impl<'a, C: Sized> ItemsPin<C>
where
	C: Items<'a>,
	C::Item: Unpin,
{
	#[inline(always)]
	pub fn new(collection: C) -> ItemsPin<C> {
		Self { collection }
	}

	/// Unwraps this [`ItemsPin<C>`], returning the underlying collection.
	///
	/// This requires that the items inside this [`ItemsPin`] are [`Unpin`],
	/// so that we can ignore the pinning invariants when unwrapping it.
	#[inline(always)]
	pub fn into_inner(items_pin: ItemsPin<C>) -> C {
		items_pin.collection
	}
}

impl<'a, C: Sized> ItemsPin<C>
where
	C: Items<'a>,
{
	/// Constructs a new [`ItemsPin<P>`] around a collection of items of a type that may or may not implement [`Unpin`].
	///
	/// If `collection` contains items of an [`Unpin`] type, [`ItemsPin::new`] should be used instead.
	///
	/// # Safety
	///
	/// See [`Pin::new_unchecked`].
	#[inline(always)]
	pub unsafe fn new_unchecked(collection: C) -> ItemsPin<C> {
		Self { collection }
	}

	/// Unwraps this `ItemsPin<C>`, returning the underlying collection.
	///
	/// # Safety
	///
	/// See [`Pin::into_inner_unchecked`].
	#[inline(always)]
	pub unsafe fn into_inner_unchecked(items_pin: ItemsPin<C>) -> C {
		items_pin.collection
	}
}

pub trait Items<'a> {
	type Item: 'a;
	type ItemsIter: 'a + Iterator<Item = &'a Self::Item>;

	fn items(&'a self) -> Self::ItemsIter;
}

pub trait ItemsPinned<'a>: Items<'a> {
	type ItemsPinnedIter: 'a + Iterator<Item = Pin<&'a Self::Item>>;

	fn items_pinned(&'a self) -> Self::ItemsPinnedIter;
}

pub trait ItemsMut<'a>: Items<'a> {
	type ItemsMutIter: 'a + Iterator<Item = &'a mut Self::Item>;

	fn items_mut(&'a mut self) -> Self::ItemsMutIter;
}

pub trait ItemsPinnedMut<'a>: ItemsPinned<'a> + ItemsMut<'a> {
	type ItemsPinnedMutIter: 'a + Iterator<Item = Pin<&'a mut Self::Item>>;

	fn items_pinned_mut(&'a mut self) -> Self::ItemsPinnedMutIter;
}

pub struct PinIter<Iter> {
	iter: Iter,
}

impl<Iter> PinIter<Iter>
where
	Iter: Iterator,
	Iter::Item: Deref,
{
	pub fn new(iter: Iter) -> Self
	where
		<Iter::Item as Deref>::Target: Unpin,
	{
		Self { iter }
	}

	pub unsafe fn new_unchecked(iter: Iter) -> Self {
		Self { iter }
	}
}

impl<Iter> Iterator for PinIter<Iter>
where
	Iter: Iterator,
	Iter::Item: Deref,
{
	type Item = Pin<Iter::Item>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.next()
			.map(|item| unsafe { Pin::new_unchecked(item) })
	}
}

impl<'a, C: ?Sized> Items<'a> for ItemsPin<C>
where
	C: Items<'a>,
{
	type Item = C::Item;
	type ItemsIter = C::ItemsIter;

	fn items(&'a self) -> Self::ItemsIter {
		self.collection.items()
	}
}

impl<'a, C: ?Sized> ItemsPinned<'a> for ItemsPin<C>
where
	C: Items<'a>,
{
	type ItemsPinnedIter = PinIter<C::ItemsIter>;

	fn items_pinned(&'a self) -> Self::ItemsPinnedIter {
		unsafe { PinIter::new_unchecked(self.collection.items()) }
	}
}

impl<'a, C: ?Sized> ItemsMut<'a> for ItemsPin<C>
where
	C: ItemsMut<'a>,
{
	type ItemsMutIter = C::ItemsMutIter;

	fn items_mut(&'a mut self) -> Self::ItemsMutIter {
		self.collection.items_mut()
	}
}

impl<'a, C: ?Sized> ItemsPinnedMut<'a> for ItemsPin<C>
where
	C: ItemsMut<'a>,
{
	type ItemsPinnedMutIter = PinIter<C::ItemsMutIter>;

	fn items_pinned_mut(&'a mut self) -> Self::ItemsPinnedMutIter {
		unsafe { PinIter::new_unchecked(self.collection.items_mut()) }
	}
}
