use super::{IntoIter, Iter, IterMut};

// ************************************************************************* //
// DoubleEndedIterator for IntoIter<V>
// ************************************************************************* //

/// An iterator able to yield elements from both ends.
///
/// Something that implements `DoubleEndedIterator` has one extra capability
/// over something that implements [`Iterator`]: the ability to also take
/// `Item`s from the back, as well as the front.
///
/// It is important to note that both back and forth work on the same range,
/// and do not cross: iteration is over when they meet in the middle.
impl<V> DoubleEndedIterator for IntoIter<V> {
    // Removes and returns an element from the end of the iterator.
    ///
    /// Returns `None` when there are no more elements.
    fn next_back(&mut self) -> Option<Self::Item> {
        // Your code here
        unimplemented!()
    }
}

// ************************************************************************* //
// DoubleEndedIterator for IterMut<'a, V>
// ************************************************************************* //

/// An iterator able to yield elements from both ends.
///
/// Something that implements `DoubleEndedIterator` has one extra capability
/// over something that implements [`Iterator`]: the ability to also take
/// `Item`s from the back, as well as the front.
///
/// It is important to note that both back and forth work on the same range,
/// and do not cross: iteration is over when they meet in the middle.
impl<'a, V> DoubleEndedIterator for IterMut<'a, V> {
    // Removes and returns an element from the end of the iterator.
    ///
    /// Returns `None` when there are no more elements.
    fn next_back(&mut self) -> Option<Self::Item> {
        // Your code here
        unimplemented!()
    }
}

// ************************************************************************* //
// DoubleEndedIterator for Iter<'a, V>
// ************************************************************************* //

/// An iterator able to yield elements from both ends.
///
/// Something that implements `DoubleEndedIterator` has one extra capability
/// over something that implements [`Iterator`]: the ability to also take
/// `Item`s from the back, as well as the front.
///
/// It is important to note that both back and forth work on the same range,
/// and do not cross: iteration is over when they meet in the middle.
impl<'a, V> DoubleEndedIterator for Iter<'a, V> {
    // Removes and returns an element from the end of the iterator.
    ///
    /// Returns `None` when there are no more elements.
    fn next_back(&mut self) -> Option<Self::Item> {
        // Your code here
        unimplemented!()
    }
}

// ************************************************************************* //
// Tests
// ************************************************************************* //

#[cfg(test)]
mod tests;
