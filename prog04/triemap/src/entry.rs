use super::TrieMap;

impl<V> TrieMap<V> {
    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: &str) -> Entry<'_, V> {
        // Your code here
        unimplemented!()
    }
}

// ************************************************************************* //
// Entry
// ************************************************************************* //

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`TrieMap`].
///
/// [`entry`]: TrieMap::entry
#[derive(Debug)]
pub enum Entry<'a, V>
where
    V: 'a,
{
    Vacant(VacantEntry<'a, V>),
    Occupied(OccupiedEntry<'a, V>),
}

use Entry::{Occupied, Vacant};

impl<'a, V> Entry<'a, V> {
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert(Default::default())
    }
    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    pub fn or_insert(self, default: V) -> &'a mut V {
        self.or_insert_with(|| default)
    }
    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the
    /// entry.
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        self.or_insert_with_key(|_| default())
    }
    /// Ensures a value is in the entry by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function a reference to the key that was moved during the `.entry(key)` method call.
    ///
    /// The reference to the moved key is provided so that cloning or copying the key is
    /// unnecessary, unlike with `.or_insert_with(|| ... )`.
    pub fn or_insert_with_key<F: FnOnce(&str) -> V>(self, default: F) -> &'a mut V {
        match self {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }
    /// Returns this entry's key.
    pub fn key(&self) -> &str {
        match self {
            Occupied(entry) => entry.key(),
            Vacant(entry) => entry.key(),
        }
    }
    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
        match self {
            Occupied(mut entry) => {
                let v = entry.get_mut();
                f(v);
                Occupied(entry)
            }
            Vacant(entry) => Vacant(entry),
        }
    }
}

// ************************************************************************* //
// OccupiedEntry
// ************************************************************************* //

/// A view into an occupied entry in a `TrieMap`.
/// It is part of the [`Entry`] enum.
#[derive(Debug)]
pub struct OccupiedEntry<'a, V>
where
    V: 'a,
{
    // Your code here

    // A bogus field to suppress compiler errors.
    _bogus: &'a V,
}

impl<'a, V> OccupiedEntry<'a, V> {
    /// Gets a reference to the key in the entry.
    pub fn key(&self) -> &str {
        // Your code here
        unimplemented!()
    }
    /// Take the ownership of the key and value from the map.
    pub fn remove_entry(self) -> (String, V) {
        // Your code here
        unimplemented!()
    }
    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        // Your code here
        unimplemented!()
    }
    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` which may outlive the
    /// destruction of the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: Self::into_mut
    pub fn get_mut(&mut self) -> &mut V {
        // Your code here
        unimplemented!()
    }
    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: Self::get_mut
    pub fn into_mut(self) -> &'a mut V {
        // Your code here
        unimplemented!();
    }
    /// Sets the value of the entry, and returns the entry's old value.
    pub fn insert(&mut self, value: V) -> V {
        // Your code here
        unimplemented!()
    }
    /// Takes the value out of the entry, and returns it.
    pub fn remove(self) -> V {
        // Your code here
        unimplemented!()
    }
}

// ************************************************************************* //
// VacantEntry
// ************************************************************************* //

/// A view into a vacant entry in a `TrieMap`.
/// It is part of the [`Entry`] enum.
#[derive(Debug)]
pub struct VacantEntry<'a, V>
where
    V: 'a,
{
    // Your code here

    // A bogus field to suppress compiler errors.
    _bogus: &'a V,
}

impl<'a, V> VacantEntry<'a, V> {
    /// Gets a reference to the key that would be used when inserting a value
    /// through the `VacantEntry`.
    pub fn key(&self) -> &str {
        // Your code here
        unimplemented!()
    }
    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns a mutable reference to it.
    pub fn insert(self, value: V) -> &'a mut V {
        // Your code here
        unimplemented!()
    }
    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns an `OccupiedEntry`.
    fn insert_entry(self, value: V) -> OccupiedEntry<'a, V> {
        // Your code here
        unimplemented!()
    }
}

// ************************************************************************* //
// Tests
// ************************************************************************* //

#[cfg(test)]
mod tests;
