#![doc(html_root_url = "https://docs.rs/napje/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::inline_always)] // `const fn` with added constraints is unstable.

use role::Role;
use std::{
	marker::PhantomData,
	ops::{Deref, DerefMut},
	pin::Pin,
};

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

pub mod std_impls;

pub mod role {
	//! Kinds of items that can be pinned, to disambiguate e.g. keys and values.

	pub trait Role {}

	pub enum Items {}
	impl Role for Items {}

	pub enum Keys {}
	impl Role for Keys {}

	pub enum Values {}
	impl Role for Values {}

	pub enum Entries {}
	impl Role for Entries {}
}

/// A pinning wrapper for a collection type `C` that can pin-project to its items while pinned this way.
///
/// Unlike when using [`Pin<&C>`](`std::pin::Pin`), this allows the collection itself to stay [`Unpin`].
///
/// [`ItemsPin<R, C>`] acts to [`C: Items<Item = T>`](`Items`) as [`Pin<P>`](`std::pin::Pin`) does to [`P: Deref<Target = T>`](`std::ops::Deref`).
///
/// `#[repr(transparent)]` towards `C`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ItemsPin<R, C: ?Sized> {
	_role: PhantomData<R>,
	collection: C,
}

//TODO: Allow creation with custom roles that don't come with any pre-implemented traits, functions or methods.

impl<'a, R: Role, C: Sized> ItemsPin<R, C>
where
	C: Items<'a, R>,
	C::Item: Unpin,
{
	#[inline(always)]
	pub fn new(collection: C) -> ItemsPin<R, C> {
		unsafe { Self::new_unchecked(collection) }
	}

	/// Unwraps this [`ItemsPin<C>`], returning the underlying collection.
	///
	/// This requires that the items inside this [`ItemsPin`] are [`Unpin`],
	/// so that we can ignore the pinning invariants when unwrapping it.
	#[inline(always)]
	pub fn into_inner(items_pin: ItemsPin<R, C>) -> C {
		items_pin.collection
	}

	#[inline(always)]
	pub fn new_ref(collection: &C) -> &ItemsPin<R, C> {
		unsafe { &*(collection as *const C).cast() }
	}

	/// Unwraps this [`ItemsPin<C>`], returning the underlying collection.
	///
	/// This requires that the items inside this [`ItemsPin`] are [`Unpin`],
	/// so that we can ignore the pinning invariants when unwrapping it.
	#[inline(always)]
	pub fn inner_ref(items_pin: &ItemsPin<R, C>) -> &C {
		&items_pin.collection
	}

	#[inline(always)]
	pub fn new_mut(collection: &mut C) -> &mut ItemsPin<R, C> {
		unsafe { &mut *(collection as *mut C).cast() }
	}

	/// Unwraps this [`ItemsPin<C>`], returning the underlying collection.
	///
	/// This requires that the items inside this [`ItemsPin`] are [`Unpin`],
	/// so that we can ignore the pinning invariants when unwrapping it.
	#[inline(always)]
	pub fn inner_mut(items_pin: &mut ItemsPin<R, C>) -> &mut C {
		&mut items_pin.collection
	}
}

impl<'a, R: Role, C: Sized> ItemsPin<R, C>
where
	C: Items<'a, R>,
{
	/// Constructs a new [`ItemsPin<P>`] around a collection of items of a type that may or may not implement [`Unpin`].
	///
	/// If `collection` contains items of an [`Unpin`] type, [`ItemsPin::new`] should be used instead.
	///
	/// # Safety
	///
	/// See [`Pin::new_unchecked`].
	#[inline(always)]
	pub unsafe fn new_unchecked(collection: C) -> ItemsPin<R, C> {
		Self {
			_role: PhantomData,
			collection,
		}
	}

	/// Unwraps this `ItemsPin<C>`, returning the underlying collection.
	///
	/// # Safety
	///
	/// See [`Pin::into_inner_unchecked`].
	#[inline(always)]
	pub unsafe fn into_inner_unchecked(items_pin: ItemsPin<R, C>) -> C {
		items_pin.collection
	}

	/// Constructs a new [`ItemsPin<P>`] around a collection of items of a type that may or may not implement [`Unpin`].
	///
	/// If `collection` contains items of an [`Unpin`] type, [`ItemsPin::new`] should be used instead.
	///
	/// # Safety
	///
	/// See [`Pin::new_unchecked`].
	#[inline(always)]
	pub unsafe fn new_ref_unchecked(collection: &C) -> &ItemsPin<R, C> {
		&*(collection as *const C).cast()
	}

	/// Unwraps this [`ItemsPin<C>`], returning the underlying collection.
	///
	/// # Safety
	///
	/// See [`Pin::into_inner_unchecked`].
	#[inline(always)]
	pub unsafe fn inner_ref_unchecked(items_pin: &ItemsPin<R, C>) -> &C {
		&items_pin.collection
	}

	/// Constructs a new [`ItemsPin<P>`] around a collection of items of a type that may or may not implement [`Unpin`].
	///
	/// If `collection` contains items of an [`Unpin`] type, [`ItemsPin::new`] should be used instead.
	///
	/// # Safety
	///
	/// See [`Pin::new_unchecked`].
	#[inline(always)]
	pub unsafe fn new_mut_unchecked(collection: &mut C) -> &mut ItemsPin<R, C> {
		&mut *(collection as *mut C).cast()
	}

	/// Unwraps this [`ItemsPin<C>`], returning the underlying collection.
	///
	/// # Safety
	///
	/// See [`Pin::into_inner_unchecked`].
	#[inline(always)]
	pub unsafe fn inner_mut_unchecked(items_pin: &mut ItemsPin<R, C>) -> &mut C {
		&mut items_pin.collection
	}
}

pub trait Items<'a, R> {
	type Item: 'a;
	type ItemsIter: 'a + Iterator<Item = &'a Self::Item>;

	fn items(&'a self) -> Self::ItemsIter;
}

pub trait ItemsPinned<'a, R>: Items<'a, R> {
	type ItemsPinnedIter: 'a + Iterator<Item = Pin<&'a Self::Item>>;

	fn items_pinned(&'a self) -> Self::ItemsPinnedIter;
}

pub trait ItemsMut<'a, R>: Items<'a, R> {
	type ItemsMutIter: 'a + Iterator<Item = &'a mut Self::Item>;

	fn items_mut(&'a mut self) -> Self::ItemsMutIter;
}

pub trait ItemsPinnedMut<'a, R>: ItemsPinned<'a, R> + ItemsMut<'a, R> {
	type ItemsPinnedMutIter: 'a + Iterator<Item = Pin<&'a mut Self::Item>>;

	fn items_pinned_mut(&'a mut self) -> Self::ItemsPinnedMutIter;
}

#[repr(transparent)]
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

	/// # Safety
	///
	/// Only safe iff all pinning invariants are upheld when each of `Iter's` [`Iterator::Item`]s is wrapped in [`Pin<_>`].
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

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.next()
			.map(|item| unsafe { Pin::new_unchecked(item) })
	}
}

impl<'a, R: Role, C: ?Sized> Items<'a, R> for ItemsPin<R, C>
where
	C: Items<'a, R>,
{
	type Item = C::Item;
	type ItemsIter = C::ItemsIter;

	fn items(&'a self) -> Self::ItemsIter {
		self.collection.items()
	}
}

impl<'a, R: Role, C: ?Sized> ItemsPinned<'a, R> for ItemsPin<R, C>
where
	C: Items<'a, R>,
{
	type ItemsPinnedIter = PinIter<C::ItemsIter>;

	fn items_pinned(&'a self) -> Self::ItemsPinnedIter {
		unsafe { PinIter::new_unchecked(self.collection.items()) }
	}
}

impl<'a, R: Role, C: ?Sized> ItemsMut<'a, R> for ItemsPin<R, C>
where
	C: ItemsMut<'a, R>,
{
	type ItemsMutIter = C::ItemsMutIter;

	fn items_mut(&'a mut self) -> Self::ItemsMutIter {
		self.collection.items_mut()
	}
}

impl<'a, R: Role, C: ?Sized> ItemsPinnedMut<'a, R> for ItemsPin<R, C>
where
	C: ItemsMut<'a, R>,
{
	type ItemsPinnedMutIter = PinIter<C::ItemsMutIter>;

	fn items_pinned_mut(&'a mut self) -> Self::ItemsPinnedMutIter {
		unsafe { PinIter::new_unchecked(self.collection.items_mut()) }
	}
}

impl<R: Role, C: ?Sized> Deref for ItemsPin<R, C> {
	type Target = C;

	fn deref(&self) -> &Self::Target {
		&self.collection
	}
}

impl<'a, R: Role, C: Items<'a, R> + ?Sized> DerefMut for ItemsPin<R, C>
where
	C::Item: Unpin,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.collection
	}
}
