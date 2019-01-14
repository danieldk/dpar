use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::Hash;

use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};

use features::addr::Source;
use guide::Guide;
use numberer::Numberer;
use system::{DependencySet, ParserState};

pub trait TransitionSystem {
    type Transition: Transition;
    type Oracle: Guide<Transition = Self::Transition>;

    /// Parser state addresses undergoing attachment.
    ///
    /// This constant stores the addresses undergoing attachment in
    /// the transition system, typically through a left-arc or
    /// right-arc transition.
    const ATTACHMENT_ADDRS: [AttachmentAddr; 2];

    fn is_terminal(state: &ParserState) -> bool;
    fn oracle(gold_dependencies: &DependencySet) -> Self::Oracle;
    fn transitions(&self) -> &TransitionLookup<Self::Transition>;
}

/// A pair of parser state addresses undergoing attachment.
///
/// Instances of this struct encode parser state addresses that
/// undergo attachment. For example, the following instance:
///
/// ~~~
/// use dpar::features::addr::Source;
/// use dpar::system::AttachmentAddr;
///
/// AttachmentAddr {
///   head: Source::Stack(0),
///   dependent: Source::Stack(1),
/// };
/// ~~~
///
/// Encodes that attachments are made between the tip and second
/// element of the stack, where the stack tip token acts as the head.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct AttachmentAddr {
    pub head: Source,
    pub dependent: Source,
}

pub trait Transition: Clone + Debug + Eq + Hash + Serialize + DeserializeOwned {
    type S: TransitionSystem;

    fn is_possible(&self, state: &ParserState) -> bool;
    fn apply(&self, state: &mut ParserState);
}

/// Transition lookup table.
///
/// Instances of this type provide a transition lookup table. When a fresh
/// table is created, a transition lookup adds a transition to the table.
/// If the table is frozen through serialization or the `freeze` method,
/// the table becomes immutable. Lookups of transitions that are not in the
/// table will result in a special index (`0`).
#[derive(Debug, Deserialize, Eq)]
pub enum TransitionLookup<T>
where
    T: Eq + Hash,
{
    Fresh(RefCell<Numberer<T>>),
    Frozen(Numberer<T>),
}

impl<T> PartialEq for TransitionLookup<T>
where
    T: Eq + Hash,
{
    fn eq(&self, rhs: &TransitionLookup<T>) -> bool {
        use self::TransitionLookup::*;

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

impl<T> Serialize for TransitionLookup<T>
where
    T: Eq + Hash + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use self::TransitionLookup::*;

        match self {
            Fresh(refcell) => serializer.serialize_newtype_variant(
                "TransitionLookup",
                1,
                "Frozen",
                &*refcell.borrow(),
            ),
            Frozen(ref numberer) => {
                serializer.serialize_newtype_variant("TransitionLookup", 1, "Frozen", numberer)
            }
        }
    }
}

impl<T> TransitionLookup<T>
where
    T: Clone + Eq + Hash,
{
    /// Freeze a transition table.
    pub fn freeze(self) -> Self {
        use self::TransitionLookup::*;

        match self {
            Fresh(cell) => Frozen(cell.into_inner()),
            frozen => frozen,
        }
    }

    /// Length of the transition table.
    pub fn len(&self) -> usize {
        use self::TransitionLookup::*;

        match self {
            Fresh(cell) => cell.borrow().len() + 1,
            Frozen(numberer) => numberer.len() + 1,
        }
    }

    /// Look up a transition.
    ///
    /// If the the transition is not in the table, it is added for a
    /// fresh table. Frozen tables will return the special identifier
    /// `0` in such cases.
    pub fn lookup(&self, t: T) -> usize {
        use self::TransitionLookup::*;

        match self {
            Fresh(cell) => cell.borrow_mut().add(t),
            Frozen(numberer) => numberer.number(&t).unwrap_or(0),
        }
    }

    /// Get the transition corresponding to an identifier.
    ///
    /// Fresh tables return copies of transitions, frozen tables references
    /// to transitions. `None` is returned when the identifier is unknown
    /// or the special identifier `0`.
    pub fn value(&self, id: usize) -> Option<Cow<T>> {
        use self::TransitionLookup::*;

        if id == 0 {
            return None;
        }

        match self {
            Fresh(cell) => cell.borrow().value(id).cloned().map(Cow::Owned),
            Frozen(numberer) => numberer.value(id).map(Cow::Borrowed),
        }
    }
}

impl<T> Default for TransitionLookup<T>
where
    T: Clone + Eq + Hash,
{
    fn default() -> Self {
        TransitionLookup::Fresh(RefCell::new(Numberer::new(1)))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use systems::arc_standard::ArcStandardTransition;

    use super::TransitionLookup;

    #[test]
    pub fn transition_lookup() {
        use self::ArcStandardTransition::*;

        let fresh = TransitionLookup::default();
        assert_eq!(fresh.lookup(Shift), 1);
        assert_eq!(fresh.lookup(LeftArc("foo".to_owned())), 2);
        assert_eq!(fresh.lookup(RightArc("bar".to_owned())), 3);

        let frozen = fresh.freeze();

        assert_eq!(frozen.len(), 4);

        assert_eq!(frozen.lookup(Shift), 1);
        assert_eq!(frozen.lookup(LeftArc("foo".to_owned())), 2);
        assert_eq!(frozen.lookup(RightArc("bar".to_owned())), 3);
        assert_eq!(frozen.lookup(LeftArc("baz".to_owned())), 0);

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

        let fresh = TransitionLookup::default();
        assert_eq!(fresh.lookup(Shift), 1);
        assert_eq!(fresh.lookup(LeftArc("foo".to_owned())), 2);
        assert_eq!(fresh.lookup(RightArc("bar".to_owned())), 3);

        let serialized =
            ::serde_yaml::to_string(&fresh).expect("Serialization of transition lookup failed");

        let frozen: TransitionLookup<ArcStandardTransition> = ::serde_yaml::from_str(&serialized)
            .expect("Deserialization of transition lookup failed");

        // Check that serialization freezes the lookup table.
        if let TransitionLookup::Fresh(_) = frozen {
            panic!("Deserialized transition lookup was fresh, should be frozen.");
        };

        // Check that serialization/deserialization roundtrip preserved data.
        assert_eq!(fresh, frozen);
    }
}
