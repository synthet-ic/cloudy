//! <https://github.com/grpc/grpc-go/tree/master/attributes>

use std::collections::HashMap;

/// Attributes is an immutable struct for storing and retrieving generic key/value pairs. Keys must be hashable, and users should define their own types for keys. Values should not be modified after they are added to an Attributes or if they were received from one.  If values implement 'Equal(o interface{}) bool', it will be called by (*Attributes).Equal to determine whether two values with the same key should be considered equal.
pub struct Attributes<K, V> {
    map: HashMap<K, V>
}

impl<K, V> Attributes<K, V> {
    /// Returns a new Attributes containing the key/value pair.
    pub fn new(key: K, value: V) -> Self {
        let map = HashMap::new();
        map.insert(key, value);
        Self { map }
    }

    /// Returns a new Attributes containing the previous keys and values and the new key/value pair. If the same key appears multiple times, the last value overwrites all previous values for that key.  To remove an existing key, use a None value.  value should not be modified later.
    pub fn with_value(&self, key: K, value: V) -> Self {
        if self.map.is_empty() {
            return Self::new(key, value)
        }
        let map = HashMap::with_capacity(self.map.len() + 1);
        let mut n = Self { map };
        for (key, value) in self.map.iter() {
            n.map.insert(key, value);
        }
        n.map.insert(key, value);
        n
    }

    /// Returns the value associated with these attributes for key, or None if no value is associated with key.  The returned value should not be modified.
    pub fn value(&self, key: K) -> Option<V> {
        self.map.get(key)
    }

    /// Equal returns whether a and o are equivalent. If `Equal(o interface{}) bool` is implemented for a value in the attributes, it is called to determine if the value matches the one stored in the other attributes. If Equal is not implemented, standard equality is used to determine if the two values are equal. Note that some types (e.g. maps) aren't comparable by default, so they must be wrapped in a struct, or in an alias type, with Equal defined.
    pub fn equal(&self, other: &Self) -> bool {
        if self.map.is_empty() && other.map.is_empty() {
            return true
        }
        if self.map.len() != self.map.len() {
            return false
        }
        for (key, value) in self.map.iter() {
            let o_value = match other.map.get(key) {
                Some(o_value) => {

                },
                None => return false
            }
            if (eq, ok) = value.(interface{ Equal(o interface{}) bool }); ok {
                if !eq.Equal(o_value) {
                    return false
                }
            } else if value != o_value {
                // Fallback to a standard equality check if Value is unimplemented.
                return false
            }
        }
        return true
    }
}
