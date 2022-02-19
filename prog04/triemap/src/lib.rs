use std::iter::{ExactSizeIterator, FromIterator, FusedIterator, Iterator};
use std::ops::Index;

/// A (private) helper function to split a [`&str`] into a first character and a
/// remaining slice.
fn str_split_first(s: &str) -> Option<(char, &str)> {
    s.chars().next().map(|c| (c, &s[c.len_utf8()..]))
}

// ************************************************************************* //
// TrieMap<V>
// ************************************************************************* //

/// A trie (also called a digital tree or prefix tree), which maps strings to
/// values.
///
/// See [`https://en.wikipedia.org/wiki/Trie`](https://en.wikipedia.org/wiki/Trie).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrieMap<V> {
    /// The total number of string/value pairs represented by the `TrieMap`,
    /// including this node and all descendant nodes.
    ///
    /// Note that this is *not* the total number of `TrieMap` nodes; rather, it
    /// is the toal number of `TrieMap` nodes that have a `Some` `val` field.
    len: usize,
    /// The value, if any, at this node of the `TrieMap`; the corresponding
    /// string is implicit, determined by the sequence of characters from the
    /// root `TrieMap` to this node.
    val: Option<V>,
    /// The children tries, as a mapping from a character to a `TrieMap`, stored
    /// in lexicographic (i.e., dictionary) order by characters; thus, it is
    /// possible to binary search the children tries by the character.
    ///
    /// As an invariant of the `TrieMap`, no child trie should be empty.
    children: Vec<(char, TrieMap<V>)>,
}

impl<V> TrieMap<V> {
    /// Creates an empty `TrieMap`.
    pub fn new() -> Self {
        TrieMap {
            len: 0,
            val: None,
            children: vec![],
        }
    }

    /// Returns the number of elements (string/value pairs) in the map.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the child trie reached by the character `c`.
    ///
    /// Hint: Use [`binary_search_by_key`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.binary_search_by_key).
    pub fn next(&self, c: char) -> Option<&Self> {
        // Your code here
        unimplemented!()
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// This method *must* be implemented with iteration (`loop`, `for`, and/or
    /// `Iterator` method(s)) and without recursion.
    ///
    /// Hint: Use [`binary_search_by_key`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.binary_search_by_key).
    pub fn get(&self, key: &str) -> Option<&V> {
        // Your code here
        unimplemented!()
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// This method *must* be implemented with iteration (`loop`, `for`, and/or
    /// `Iterator` method(s)) and without recursion.
    ///
    /// Hint: Use [`binary_search_by_key`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.binary_search_by_key).
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        // Your code here
        unimplemented!()
    }

    /// Inserts a string/value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    ///
    /// For full credit, this method *must* be implemented with iteration
    /// (`loop`, `for`, and/or `Iterator` method(s)) and without recursion.  For
    /// (significant) partial credit, this method may be implemented with
    /// recursion.  Implementing [`Self::insert`] with iteration is a little
    /// more challenging than implementing [`Self::get`] and [`Self::get_mut`]
    /// with iteration.
    ///
    /// Hint: Be sure to maintain the `TrieMap` invariants, especially that the
    /// `len` field caches the total number of elements in the map.
    ///
    /// Hint: Use [`binary_search_by_key`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.binary_search_by_key).
    ///
    /// Hint: Use [`std::mem::replace`](https://doc.rust-lang.org/stable/std/mem/fn.replace.html).
    ///
    /// Hint: When implementing this method with iteration, it is acceptable
    /// (and expected) that a temporary (heap-allocated) data structure is used.
    /// It is also acceptable (and expected) that multiple iteration constructs
    /// are used.  The most important aspect of implementing this method with
    /// iteration is that no extraneous "expensive" operations (especially not
    /// [`binary_search_by_key`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.binary_search_by_key)
    /// and, ideally, neither
    /// [`Vec::index`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.index)
    /// nor
    /// [`Vec::index_mut`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.index)
    /// that incur bounds checks) are performed.
    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        // Your code here
        unimplemented!()
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// This method may be implemented with recursion.
    ///
    /// Hint: Be sure to maintain the `TrieMap` invariants, especially that the
    /// `len` field caches the total number of elements in the map and that no
    /// child trie should be empty.
    ///
    /// Hint: Use [`binary_search_by_key`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.binary_search_by_key).
    ///
    /// Hint: Use [`std::mem::replace`](https://doc.rust-lang.org/stable/std/mem/fn.replace.html).
    ///
    /// Challenge: Implement this methods with iteration (`loop`, `for`, and/or
    /// `Iterator` method(s)) and without recursion (and without `unsafe`).
    /// Implementing [`Self::remove`] with iteration is much more challenging
    /// than implementing [`Self::insert`] with iteration.
    ///
    /// Hint: When implementing this method with iteration, it is acceptable
    /// (and expected) that a temporary (heap-allocated) data structure is used.
    /// It is also acceptable (and expected) that multiple iteration constructs
    /// are used.  The most important aspect of implementing this method with
    /// iteration is that no extraneous "expensive" operations (especially not
    /// [`binary_search_by_key`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.binary_search_by_key)
    /// and, ideally, neither
    /// [`Vec::index`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.index)
    /// nor
    /// [`Vec::index_mut`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.index)
    /// that incur bounds checks) are performed.
    ///
    /// The instructor's (safe) reference solution does incur an extraneous
    /// bounds check due to
    /// [`Vec::index`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.index);
    /// this could be eliminated using the `unsafe`
    /// [`[T]::get_unchecked`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked).
    /// Another `unsafe` solution, closer in spirit to that of [`Self::insert`],
    /// uses raw pointers (see [Dereferencing a Raw
    /// Pointer](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer)).
    ///
    /// Why doesn't the technique used to implement [`Self::get`],
    /// [`Self::get_mut`], and [`Self::insert`] without recursion suffice to
    /// implement [`Self::remove`] without recursion?  What does recursion
    /// establish that is not captured by iteration?
    pub fn remove(&mut self, key: &str) -> Option<V> {
        // Your code here
        unimplemented!()
    }
}

/// A trait for giving a type a useful default value.
impl<V> Default for TrieMap<V> {
    /// Creates an empty `TrieMap<V>`.
    fn default() -> TrieMap<V> {
        TrieMap::new()
    }
}

/// Used for indexing operations (`container[index]`) in immutable contexts.
///
/// `container[index]` is actually syntactic sugar for
/// `*container.index(index)`, but only when used as an immutable value.  This
/// allows nice things such as `let value = v[index]` if the type of `value`
/// implements [`Copy`].
impl<V> Index<&str> for TrieMap<V> {
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `TrieMap`.
    fn index(&self, key: &str) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

/// Conversion from an [`Iterator`].
///
/// By implementing `FromIterator` for a type, you define how it will be
/// created from an iterator. This is common for types which describe a
/// collection of some kind.
impl<'a, V> FromIterator<(&'a str, V)> for TrieMap<V> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'a str, V)>,
    {
        let mut tm = TrieMap::new();
        tm.extend(iter);
        tm
    }
}

/// Extend a collection with the contents of an iterator.
///
/// Iterators produce a series of values, and collections can also be thought
/// of as a series of values. The `Extend` trait bridges this gap, allowing you
/// to extend a collection by including the contents of that iterator. When
/// extending a collection with an already existing key, that entry is updated
/// or, in the case of collections that permit multiple entries with equal
/// keys, that entry is inserted.
impl<'a, V> Extend<(&'a str, V)> for TrieMap<V> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (&'a str, V)>,
    {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

/// Extend a collection with the contents of an iterator.
///
/// Iterators produce a series of values, and collections can also be thought
/// of as a series of values. The `Extend` trait bridges this gap, allowing you
/// to extend a collection by including the contents of that iterator. When
/// extending a collection with an already existing key, that entry is updated
/// or, in the case of collections that permit multiple entries with equal
/// keys, that entry is inserted.
impl<'a, V: 'a> Extend<(&'a str, &'a V)> for TrieMap<V>
where
    V: Copy,
{
    /// Inserts all new string/value pairs from the iterator and replaces values
    /// with existing keys with new values returned from the iterator.
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (&'a str, &'a V)>,
    {
        for (k, v) in iter {
            self.insert(k, *v);
        }
    }
}

// ************************************************************************* //
// IntoIter<V>
// ************************************************************************* //

/// An owning iterator over the entries of a `TrieMap`.
///
/// This `struct` is created by the [`into_iter`] method on [`TrieMap`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: IntoIterator::into_iter
pub struct IntoIter<V> {
    // Your code here

    // A bogus field to supress compiler errors.
    _bogus: V,
}
/// Conversion into an [`Iterator`].
///
/// By implementing `IntoIterator` for a type, you define how it will be
/// converted to an iterator. This is common for types which describe a
/// collection of some kind.
///
/// One benefit of implementing `IntoIterator` is that your type will [work with
/// Rust's `for` loop
/// syntax](https://doc.rust-lang.org/stable/std/iter/index.html#for-loops-and-intoiterator).
impl<V> IntoIterator for TrieMap<V> {
    type Item = (String, V);
    type IntoIter = IntoIter<V>;
    /// Creates a consuming iterator, that is, one that moves each string/value
    /// pair out of the map in lexicographic (i.e., dictionary) order. The map
    /// cannot be used after calling this.
    ///
    /// The iterator `Item` type is `(String, V)`.
    ///
    /// This method must be *O*(*1*).
    fn into_iter(self: TrieMap<V>) -> Self::IntoIter {
        // Your code here
        unimplemented!()
    }
}
impl<V> Iterator for IntoIter<V> {
    type Item = (String, V);
    /// Advances the iterator and returns the next value.
    ///
    /// Returns [`None`] when iteration is finished.
    ///
    /// Because this type implements [`FusedIterator`], the implementation of
    /// `next` *must* guarantee to always return [`None`] after it has returned
    /// [`None`] once.
    ///
    /// This method should be implemented with a `loop` and without recursion.
    ///
    /// This method should be _amortized_ *O*(*1*).
    ///
    /// Hint: Recall that [`String`] implements [`Clone`].
    fn next(&mut self) -> Option<Self::Item> {
        // Your code here
        unimplemented!()
    }
    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element
    /// is the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an [`Option`]`<`[`usize`]`>`.
    /// A [`None`] here means that either there is no known upper bound, or the
    /// upper bound is larger than [`usize`].
    ///
    /// Because this type implements [`ExactSizeIterator`], the implementation
    /// of `size_hint` *must* return the exact size of the iterator.
    ///
    /// This method must be *O*(*1*).
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Your code here
        unimplemented!()
    }
}
/// An iterator that knows its exact length.
///
/// When implementing an `ExactSizeIterator`, you must also implement
/// [`Iterator`]. When doing so, the implementation of [`Iterator::size_hint`]
/// *must* return the exact size of the iterator.
impl<V> ExactSizeIterator for IntoIter<V> {
    /* default methods suffice */
}
/// An iterator that always continues to yield `None` when exhausted.
///
/// Calling next on a fused iterator that has returned `None` once is guaranteed
/// to return [`None`] again.
impl<V> FusedIterator for IntoIter<V> {
    /* marker trait; no methods */
}

// ************************************************************************* //
// IterMut<'a,V>
// ************************************************************************* //

/// A mutable iterator over the entries of a `TrieMap`.
///
/// This `struct` is created by the [`iter_mut`] method on [`TrieMap`]. See its
/// documentation for more.
///
/// [`iter_mut`]: TrieMap::iter_mut
/// An iterator over the entries of a `TrieMap`.
///
/// This `struct` is created by the [`iter`] method on [`TrieMap`]. See its
/// documentation for more.
///
/// [`iter`]: TrieMap::iter
pub struct IterMut<'a, V> {
    // Your code here

    // A bogus field to supress compiler errors.
    _bogus: &'a mut V,
}
impl<V> TrieMap<V> {
    /// An iterator visiting all string/value pairs in lexicographic (i.e.,
    /// dictionary) order.
    ///
    /// The iterator `Item` type is `(String, &'a V)`.
    ///
    /// This method must be *O*(*1*).
    pub fn iter_mut(&mut self) -> IterMut<V> {
        // Your code here
        unimplemented!()
    }
}
impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (String, &'a mut V);
    /// Advances the iterator and returns the next value.
    ///
    /// Returns [`None`] when iteration is finished.
    ///
    /// Because this type implements [`FusedIterator`], the implementation of
    /// `next` *must* guarantee to always return [`None`] after it has returned
    /// [`None`] once.
    ///
    /// This method should be implemented with a `loop` and without recursion.
    ///
    /// This method should be _amortized_ *O*(*1*).
    ///
    /// Hint: Recall that [`String`] implements [`Clone`].
    fn next(&mut self) -> Option<Self::Item> {
        // Your code here
        unimplemented!()
    }
    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element
    /// is the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an [`Option`]`<`[`usize`]`>`.
    /// A [`None`] here means that either there is no known upper bound, or the
    /// upper bound is larger than [`usize`].
    ///
    /// Because this type implements [`ExactSizeIterator`], the implementation
    /// of `size_hint` *must* return the exact size of the iterator.
    ///
    /// This method must be *O*(*1*).
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Your code here
        unimplemented!()
    }
}
/// An iterator that knows its exact length.
///
/// When implementing an `ExactSizeIterator`, you must also implement
/// [`Iterator`]. When doing so, the implementation of [`Iterator::size_hint`]
/// *must* return the exact size of the iterator.
impl<'a, V> ExactSizeIterator for IterMut<'a, V> {
    /* default methods suffice */
}
/// An iterator that always continues to yield `None` when exhausted.
///
/// Calling next on a fused iterator that has returned `None` once is guaranteed
/// to return [`None`] again.
impl<'a, V> FusedIterator for IterMut<'a, V> {
    /* marker trait; no methods */
}
/// Conversion into an [`Iterator`].
///
/// By implementing `IntoIterator` for a type, you define how it will be
/// converted to an iterator. This is common for types which describe a
/// collection of some kind.
///
/// One benefit of implementing `IntoIterator` is that your type will [work with
/// Rust's `for` loop
/// syntax](https://doc.rust-lang.org/stable/std/iter/index.html#for-loops-and-intoiterator).
impl<'a, V> IntoIterator for &'a mut TrieMap<V> {
    type Item = (String, &'a mut V);
    type IntoIter = IterMut<'a, V>;
    fn into_iter(self: &'a mut TrieMap<V>) -> Self::IntoIter {
        self.iter_mut()
    }
}

// ************************************************************************* //
// Iter<'a,V>
// ************************************************************************* //

/// An iterator over the entries of a `TrieMap`.
///
/// This `struct` is created by the [`iter`] method on [`TrieMap`]. See its
/// documentation for more.
///
/// [`iter`]: TrieMap::iter
pub struct Iter<'a, V> {
    // Your code here

    // A bogus field to supress compiler errors.
    _bogus: &'a V,
}
impl<V> TrieMap<V> {
    /// An iterator visiting all string/value pairs in lexicographic (i.e.,
    /// dictionary) order.
    ///
    /// The iterator `Item` type is `(String, &'a V)`.
    ///
    /// This method must be *O*(*1*).
    pub fn iter(&self) -> Iter<V> {
        // Your code here
        unimplemented!()
    }
}
impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (String, &'a V);
    /// Advances the iterator and returns the next value.
    ///
    /// Returns [`None`] when iteration is finished.
    ///
    /// Because this type implements [`FusedIterator`], the implementation of
    /// `next` *must* guarantee to always return [`None`] after it has returned
    /// [`None`] once.
    ///
    /// This method should be implemented with iteration (`loop`) and without
    /// recursion.
    ///
    /// This method should be _amortized_ *O*(*1*).
    ///
    /// Hint: Recall that [`String`] implements [`Clone`].
    fn next(&mut self) -> Option<Self::Item> {
        // Your code here
        unimplemented!()
    }
    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element is
    /// the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an
    /// [`Option`]`<`[`usize`]`>`.  A [`None`] here means that either there is
    /// no known upper bound, or the upper bound is larger than [`usize`].
    ///
    /// Because this type implements [`ExactSizeIterator`], the implementation
    /// of `size_hint` *must* return the exact size of the iterator.
    ///
    /// This method must be *O*(*1*).
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Your code here
        unimplemented!()
    }
}
/// An iterator that knows its exact length.
///
/// When implementing an `ExactSizeIterator`, you must also implement
/// [`Iterator`]. When doing so, the implementation of [`Iterator::size_hint`]
/// *must* return the exact size of the iterator.
impl<'a, V> ExactSizeIterator for Iter<'a, V> {
    /* default methods suffice */
}
/// An iterator that always continues to yield `None` when exhausted.
///
/// Calling next on a fused iterator that has returned `None` once is guaranteed
/// to return [`None`] again.
impl<'a, V> FusedIterator for Iter<'a, V> {
    /* marker trait; no methods */
}
/// Conversion into an [`Iterator`].
///
/// By implementing `IntoIterator` for a type, you define how it will be
/// converted to an iterator. This is common for types which describe a
/// collection of some kind.
///
/// One benefit of implementing `IntoIterator` is that your type will [work with
/// Rust's `for` loop
/// syntax](https://doc.rust-lang.org/stable/std/iter/index.html#for-loops-and-intoiterator).
impl<'a, V> IntoIterator for &'a TrieMap<V> {
    type Item = (String, &'a V);
    type IntoIter = Iter<'a, V>;
    fn into_iter(self: &'a TrieMap<V>) -> Self::IntoIter {
        self.iter()
    }
}

// ************************************************************************* //
// Discussion Prompts
// ************************************************************************* //

/*

1. Comment on the amount of code duplication in your implementation of the
`TrieMap` library.  Explain why (with the current features of the Rust language)
it is not possible to share more code.

YOUR ANSWER HERE

*/

/*

2. Explain why the various iterators over string/value pairs return a [`String`]
rather than a [`&str`].

YOUR ANSWER HERE

*/

// ************************************************************************* //
// Tests
// ************************************************************************* //

// Tests
#[cfg(test)]
#[allow(clippy::type_complexity)]
mod tests;

// ************************************************************************* //
// DoubleEndedIterator Challenge
// ************************************************************************* //

// `any()` is equivalent to `false`; remove the `#[cfg(any())]` attribute to use
// the `doubleendediterator` sub-module.
#[cfg(any())]
mod doubleendediterator;

// ************************************************************************* //
// Entry API Challenge
// ************************************************************************* //

// `any()` is equivalent to `false`; remove the `#[cfg(any())]` attribute to use
// the `entry` sub-module.
#[cfg(any())]
mod entry;
