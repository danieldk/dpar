use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

/// Numberer for categorical values, such as features or class labels.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Numberer<T>
where
    T: Eq + Hash,
{
    values: Vec<T>,
    numbers: HashMap<T, usize>,
    start_at: usize,
}

impl<T> Numberer<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new(start_at: usize) -> Self {
        Numberer {
            values: Vec::new(),
            numbers: HashMap::new(),
            start_at: start_at,
        }
    }

    /// Add an value. If the value has already been encountered before,
    /// the corresponding number is returned.
    pub fn add(&mut self, value: T) -> usize {
        match self.numbers.entry(value.clone()) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                let number = self.values.len() + self.start_at;
                self.values.push(value);
                e.insert(number);
                number
            }
        }
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

    /// Get the starting index for numbering.
    pub fn start_at(&self) -> usize {
        self.start_at
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

    /// Return the numbered values.
    pub fn values(&self) -> &[T] {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use Numberer;

    #[test]
    pub fn start_at_1() {
        let val = String::from("test");
        let mut numberer = Numberer::new(1);
        let idx = numberer.add(val.clone());

        assert_eq!(Some(&val), numberer.value(idx));
        assert_eq!(Some(idx), numberer.number(&val));
    }

    #[test]
    pub fn numberer_len() {
        let mut numberer = Numberer::new(1);
        numberer.add("hello");
        numberer.add("world");
        numberer.add("hello");

        assert_eq!(2, numberer.len());
        assert_eq!(1, numberer.start_at());
    }
}
