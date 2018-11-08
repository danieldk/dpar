//! Lookup tables

use std::borrow::{Borrow, Cow};
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::hash::Hash;

use serde::{Serialize, Serializer};

/// Trait for lookup tables.
///
/// This trait is implemented for every type that implements
/// `LookupLen` and `LookupValue`.
pub trait Lookup<B>: LookupLen + LookupValue<B>
where
    B: ?Sized + ToOwned,
{
}

impl<T, B> Lookup<B> for T
where
    B: ?Sized + ToOwned,
    T: LookupLen + LookupValue<B>,
{
}

/// Trait for lookup table length.
pub trait LookupLen {
    /// Size of the lookup table.
    fn len(&self) -> usize;
}

/// Trait for lookuping up values and identifiers.
pub trait LookupValue<B>
where
    B: ?Sized + ToOwned,
{
    /// Look up the identifier of a value.
    ///
    /// This function should always return an identifier. For mutable
    /// tables it typically generates a new identifier, whereas for
    /// immutable tables it will return a special, reserved value.
    fn lookup(&self, t: &B) -> usize;

    /// Get the value corresponding to an identifier.
    ///
    /// `None` is returned when the identifier is unknown. Otherwise,
    /// a copy of or reference to a value is returned.
    fn value(&self, id: usize) -> Option<Cow<B>>;
}

#[derive(Debug, Deserialize, Eq)]
enum FreezableLookupInner<T>
where
    T: Eq + Hash,
{
    Fresh(RefCell<Numberer<T>>),
    Frozen(Numberer<T>),
}

impl<T> PartialEq for FreezableLookupInner<T>
where
    T: Eq + Hash,
{
    fn eq(&self, rhs: &FreezableLookupInner<T>) -> bool {
        use self::FreezableLookupInner::*;

        // Two TransitionLookups are equal if their numberers
        // are equal.
        match (self, rhs) {
            (Frozen(ln), Frozen(rn)) => ln == rn,
            (Fresh(lrc), Fresh(rrc)) => lrc == rrc,
            (Frozen(ln), Fresh(rrc)) => ln == &*rrc.borrow(),
            (Fresh(lrc), Frozen(rn)) => &*lrc.borrow() == rn,
        }
    }
}

/// Freezable lookup table.
///
/// Instances of this type provide a lookup table. When a fresh table is
/// created, a lookup adds a value to the table when it does not exist.
/// If the table is frozen using the `freeze` method, the table becomes
/// immutable. Lookups of values that are not in the table will result
/// in a special identifier (`0`).
///
/// Besides the `freeze` method serialization also freezes the table:
///
/// ```
/// extern crate dpar;
/// extern crate serde_yaml;
///
/// use dpar::lookup::{FreezableLookup, LookupValue};
///
/// let fresh: FreezableLookup<String> = FreezableLookup::default();
/// let serialized = ::serde_yaml::to_string(&fresh)
///   .expect("Serialization of transition lookup failed");
/// let frozen: FreezableLookup<String> = ::serde_yaml::from_str(&serialized)
///   .expect("Deserialization of transition lookup failed");
/// assert!(frozen.is_frozen());
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct FreezableLookup<T>(FreezableLookupInner<T>)
where
    T: Eq + Hash;

impl<T> FreezableLookup<T>
where
    T: Eq + Hash,
{
    /// Check whether the table is frozen.
    pub fn is_frozen(&self) -> bool {
        use self::FreezableLookupInner::*;

        match &self.0 {
            Fresh(_) => false,
            Frozen(_) => true,
        }
    }
}

impl<T> FreezableLookup<T>
where
    T: Eq + Hash,
{
    /// Freeze a transition table.
    ///
    /// This results in a table that returns the identifier `0` for
    /// unknown values.
    ///
    /// ```
    /// use dpar::lookup::{FreezableLookup, LookupValue};
    ///
    /// let lookup = FreezableLookup::default();
    /// assert_eq!(lookup.lookup("hello"), 1);
    /// let frozen = lookup.freeze();
    /// assert_eq!(frozen.lookup("hello"), 1);
    /// assert_eq!(frozen.lookup("world"), 0);
    /// ```
    pub fn freeze(self) -> Self {
        use self::FreezableLookupInner::*;

        let inner = match self.0 {
            Fresh(cell) => Frozen(cell.into_inner()),
            frozen => frozen,
        };

        FreezableLookup(inner)
    }

    /// Thaw a transition table.
    ///
    /// This results in a table that adds unknown values when they
    /// are looked up.
    ///
    /// ```
    /// use dpar::lookup::{FreezableLookup, LookupValue};
    ///
    /// let lookup = FreezableLookup::default().freeze();
    /// assert_eq!(lookup.lookup("hello"), 0);
    /// let thawed = lookup.thaw();
    /// assert_eq!(thawed.lookup("hello"), 1);
    /// ```
    pub fn thaw(self) -> Self {
        use self::FreezableLookupInner::*;

        let inner = match self.0 {
            Frozen(numberer) => Fresh(numberer.into()),
            fresh => fresh,
        };

        FreezableLookup(inner)
    }
}

impl<T> Default for FreezableLookup<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        FreezableLookup(FreezableLookupInner::Fresh(RefCell::new(Numberer::new(1))))
    }
}

impl<O> LookupLen for FreezableLookup<O>
where
    O: Eq + Hash,
{
    fn len(&self) -> usize {
        use self::FreezableLookupInner::*;

        match &self.0 {
            Fresh(cell) => cell.borrow().len() + 1,
            Frozen(numberer) => numberer.len() + 1,
        }
    }
}

impl<B, O> LookupValue<B> for FreezableLookup<O>
where
    B: ?Sized + Eq + Hash + ToOwned<Owned = O>,
    O: Borrow<B> + Clone + Eq + Hash,
{
    fn lookup(&self, t: &B) -> usize {
        use self::FreezableLookupInner::*;

        match &self.0 {
            Fresh(cell) => cell.borrow_mut().add(t),
            Frozen(numberer) => numberer.number(t).unwrap_or(0),
        }
    }

    fn value(&self, id: usize) -> Option<Cow<B>> {
        use self::FreezableLookupInner::*;

        if id == 0 {
            return None;
        }

        match &self.0 {
            Fresh(cell) => cell.borrow().value(id).map(|v| Cow::Owned(v.to_owned())),
            Frozen(numberer) => numberer.value(id).map(|v| Cow::Borrowed(v.borrow())),
        }
    }
}

impl<T> Serialize for FreezableLookup<T>
where
    T: Eq + Hash + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use self::FreezableLookupInner::*;

        match &self.0 {
            Fresh(refcell) => serializer.serialize_newtype_variant(
                "FreezableLookup",
                1,
                "Frozen",
                &*refcell.borrow(),
            ),
            Frozen(ref numberer) => {
                serializer.serialize_newtype_variant("FreezableLookup", 1, "Frozen", numberer)
            }
        }
    }
}

/// Numberer for categorical values, such as features or class labels.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Numberer<T>
where
    T: Eq + Hash,
{
    values: Vec<T>,
    numbers: HashMap<T, usize>,
    start_at: usize,
}

impl<T> Numberer<T>
where
    T: Eq + Hash,
{
    pub fn new(start_at: usize) -> Self {
        Numberer {
            values: Vec::new(),
            numbers: HashMap::new(),
            start_at,
        }
    }

    /// Add an value. If the value has already been encountered before,
    /// the corresponding number is returned.
    pub fn add<B>(&mut self, value: &B) -> usize
    where
        T: Borrow<B>,
        B: ?Sized + Eq + Hash + ToOwned<Owned = T>,
    {
        // We cannot use the entry API because we take borrowed values,
        // requiring two lookups when a value is not numbered yet. This
        // is not a problem in practice: successful lookups are much
        // more frequent in the parser. By taking borrowed values, we
        // avoid a large amount of cloning.

        if let Some(id) = self.numbers.get(value) {
            return *id;
        }

        let number = self.values.len() + self.start_at;
        self.values.push(value.to_owned());
        self.numbers.insert(value.to_owned(), number);
        number
    }

    /// Get the number of different items numbered.
    ///
    /// If the numberer started at *2* and this method returns *5*,
    /// then the first item was numbered *2* and the last *6*.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Return the number for a value.
    pub fn number<Q>(&self, item: &Q) -> Option<usize>
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.numbers.get(item).cloned()
    }

    /// Return the value for a number.
    pub fn value(&self, number: usize) -> Option<&T> {
        assert!(
            number >= self.start_at,
            "incorrect number: {}, Numberer starts at: {}",
            number,
            self.start_at
        );
        self.values.get(number - self.start_at)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use systems::arc_standard::ArcStandardTransition;

    use super::{FreezableLookup, LookupLen, LookupValue};

    #[test]
    pub fn freezable_lookup() {
        use self::ArcStandardTransition::*;

        let fresh = FreezableLookup::default();
        assert_eq!(fresh.lookup(&Shift), 1);
        assert_eq!(fresh.lookup(&LeftArc("foo".to_owned())), 2);
        assert_eq!(fresh.lookup(&RightArc("bar".to_owned())), 3);

        let frozen = fresh.freeze();

        assert_eq!(frozen.len(), 4);
        assert_eq!(frozen.lookup(&Shift), 1);
        assert_eq!(frozen.lookup(&LeftArc("foo".to_owned())), 2);
        assert_eq!(frozen.lookup(&RightArc("bar".to_owned())), 3);
        assert_eq!(frozen.lookup(&LeftArc("baz".to_owned())), 0);

        assert_eq!(frozen.value(1), Some(Cow::Owned(Shift)));
        assert_eq!(frozen.value(2), Some(Cow::Owned(LeftArc("foo".to_owned()))));
        assert_eq!(
            frozen.value(3),
            Some(Cow::Owned(RightArc("bar".to_owned())))
        );
        assert_eq!(frozen.value(0), None);
    }

    #[test]
    pub fn transition_lookup_serialization_roundtrip() {
        use self::ArcStandardTransition::*;

        let fresh = FreezableLookup::default();
        assert_eq!(fresh.lookup(&Shift), 1);
        assert_eq!(fresh.lookup(&LeftArc("foo".to_owned())), 2);
        assert_eq!(fresh.lookup(&RightArc("bar".to_owned())), 3);

        let serialized = ::serde_yaml::to_string(&fresh).expect("Serialization of lookup failed");

        let frozen: FreezableLookup<ArcStandardTransition> =
            ::serde_yaml::from_str(&serialized).expect("Deserialization of lookup failed");

        // Check that serialization freezes the lookup table.
        assert!(frozen.is_frozen(), "Deserialized lookup should be frozen");

        // Check that serialization/deserialization roundtrip preserved data.
        assert_eq!(fresh, frozen);
    }
}
