/*!
<https://github.com/grpc/grpc-go/blob/master/metadata/metadata.go>
*/

use std::{
    collections::HashMap,
    task::Context
};

/// Metadata is a mapping from metadata keys to values. Users should use the following two convenience functions New and Pairs to generate Metadata.
pub type Metadata = HashMap<String, Vec<String>>;

/**
Pairs returns an Metadata formed by the mapping of key, value ...
Pairs panics if len(kv) is odd.

Only the following ASCII characters are allowed in keys:
- digits: 0-9
- uppercase letters: A-Z (normalized to lower)
- lowercase letters: a-z
- special characters: -_.

Uppercase letters are automatically converted to lowercase.

Keys beginning with "grpc-" are reserved for grpc-internal use only and may result in errors if set in metadata.
*/
pub fn pairs(kv: Vec<String>) -> Metadata {
    if kv.len() % 2 == 1 {
        panic!(format!("metadata: Pairs got the odd number of input pairs for metadata: {}", kv.len()));
    }
    let output = Metadata::with_capacity(kv.len() / 2);
    for i in (0..kv.len()).step_by(2) {
        let key = kv[i].to_lowercase();
        if let Some(current) = output.get_mut(&key) {
            *current.append(&mut kv[i + 1]);
        }
    }
    output
}

impl Metadata {
    /**
    New creates an Metadata from a given key-value map.

    Only the following ASCII characters are allowed in keys:
    - digits: 0-9
    - uppercase letters: A-Z (normalized to lower)
    - lowercase letters: a-z
    - special characters: -_.

    Uppercase letters are automatically converted to lowercase.

    Keys beginning with "grpc-" are reserved for grpc-internal use only and may result in errors if set in metadata.
    */
    pub fn new(m: HashMap<String, String>) -> Self {
        let output = Metadata::with_capacity(m.len());
        for (key, value) in m.iter() {
            let key = key.to_lowercase();
            output.insert(key, append(output[key], value));
        }
        output
    }

    /// Len returns the number of items in metadata.
    pub fn len(&self) -> usize {
        self.len()
    }

    /// Returns a copy of metadata.
    pub fn copy(&self) -> Self {
        join(self)
    }

    /**
    Obtains the values for a given key.

    k is converted to lowercase before searching in metadata.
    */
    pub fn get(&self, key: String) -> Option<Vec<String>> {
        let key = key.to_lowercase();
        self.get(&key)
    }

    /**
    Sets the value of a given key with a slice of values.

    k is converted to lowercase before storing in metadata.
    */
    pub fn set(&self, key: String, value: Vec<String>) {
        if value.is_empty() {
            return
        }
        let key = key.to_lowercase();
        self.insert(key, value);
    }

    /**
    Append adds the values to key k, not overwriting what was already stored at that key.

    k is converted to lowercase before storing in metadata.
    */
    pub fn append(&self, key: String, value: Vec<String>) {
        if value.is_empty() {
            return
        }
        let key = key.to_lowercase();
        if let Some(current) = self.get_mut(&key) {
            *current.append(&mut value);
        }
    }

    /**
    Removes the values for a given key k which is converted to lowercase before removing it from metadata.
    */
    pub fn delete(&mut self, key: String) {
        let key = key.to_lowercase();
        self.remove(&key);
    }
}

/**
Joins any number of mds into a single Metadata.

The order of values for each key is determined by the order in which the mds containing those values are presented to Join.
*/
pub fn join(mds: Vec<Metadata>) -> Metadata {
    let mut output = Metadata::new();
    for metadata in mds.iter() {
        for (key, value) in metadata.iter() {
            if let Some(current) = output.get_mut(&key) {
                *current.append(&mut value);
            } else {
                output.insert(key, value);
            }
        }
    }
    output
}

struct mdIncomingKey;
struct mdOutgoingKey;

/// Creates a new context with incoming metadata attached.
pub fn new_incoming_context(context: Context, metadata: Metadata) -> Context {
      return context.WithValue(context, mdIncomingKey{}, metadata)
}

/**
Creates a new context with outgoing metadata attached. If used in conjunction with append_to_outgoing_context, new_outgoing_context will overwrite any previously-appended metadata.
*/
pub fn new_outgoing_context(context: Context, metadata: Metadata) -> Context {
      context.WithValue(context, mdOutgoingKey{}, RawMetadata { metadata })
}

/**
Returns a new context with the provided kv merged with any existing metadata in the context. Please refer to the documentation
of Pairs for a description of kv.
*/
pub fn append_to_outgoing_context(context: Context, kv: Vec<String>) -> Context {
    if kv.len() % 2 == 1 {
        panic!(format!("metadata: append_to_outgoing_context got an odd number of input pairs for metadata: {}", kv.len()));
    }
    let metadata, _ = context.Value(mdOutgoingKey{}).(RawMetadata);
    let added = Vec::with_capacity(metadata.added.len() + 1);
    copy(added, metadata.added);
    added[added.len() - 1] = Vec::with_capacity(kv.len());
    copy(added[added.len() - 1], kv);
    context.WithValue(context, mdOutgoingKey{}, RawMetadata { metadata: metadata.metadata, added })
}

/**
Returns the incoming metadata in context if it exists.

All keys in the returned Metadata are lowercase.
*/
pub fn from_incoming_context(context: Context) -> Option<Metadata> {
    let metadata, ok = context.Value(mdIncomingKey{}).(Metadata);
    if !ok {
        return None
    }
    let output = Metadata::with_capacity(metadata.len());
    for (key, value) in metadata.iter() {
        // We need to manually convert all keys to lower case, because Metadata is a map, and there's no guarantee that the Metadata attached to the context is created using our helper functions.
        let key = key.to_lowercase();
        output[key] = copy_of(value);
    }
    return Some(output)
}

/**
Returns the metadata value corresponding to the metadata key from the incoming metadata if it exists. Key must be lower-case.

# Experimental

Notice: This API is EXPERIMENTAL and may be changed or removed in a later release.
*/
pub fn value_from_incoming_context(context: Context, key: String)
-> Option<Vec<String>> {
    let metadata, ok = context.Value(mdIncomingKey{}).(Metadata);
    if !ok {
        return None
    }

    if v, ok = metadata[key]; ok {
        return copy_of(v)
    }
    for (k, v) in metadata.iter() {
        // We need to manually convert all keys to lower case, because Metadata is a map, and there's no guarantee that the Metadata attached to the context is created using our helper functions.
        if k.to_lowercase() == key {
            return copy_of(v)
        }
    }
    None
}

// the returned slice must not be modified in place
fn copy_of(value: Vec<String>) -> Vec<String> {
    let output = Vec::with_capacity(value.len());
    copy(output, value);
    output
}

/**
Returns the un-merged, intermediary contents of RawMetadata.

Remember to perform strings.to_lowercase on the keys, for both the returned Metadata (Metadata is a map, there's no guarantee it's created using our helper functions) and the extra kv pairs (append_to_outgoing_context doesn't turn them into
lowercase).

This is intended for gRPC-internal use ONLY. Users should use from_outgoing_context instead.
*/
pub fn from_outgoing_context_raw(context: Context) -> Option<(Metadata, Vec<Vec<String>>)> {
    let raw, ok = context.Value(mdOutgoingKey{}).(RawMetadata);
    if !ok {
        return None
    }
    Some((raw.metadata, raw.added))
}

/**
Returns the outgoing metadata in context if it exists.

All keys in the returned Metadata are lowercase.
*/
pub fn from_outgoing_context(context: Context) -> Option<Metadata> {
    let raw, ok = context.Value(mdOutgoingKey{}).(RawMetadata);
    if !ok {
        return None
    }

    let md_size = raw.metadata.len();
    for i in raw.added {
        md_size += raw.added[i].len() / 2;
    }

    let output = Metadata::with_capacity(md_size);
    for (key, value) in raw.metadata {
        // We need to manually convert all keys to lower case, because Metadata is a map, and there's no guarantee that the Metadata attached to the context is created using our helper functions.
        let key = key.to_lowercase();
        output.insert(key, copy_of(value));
    }
    for added in raw.added.iter() {
        if added.len() % 2 == 1 {
            panic!(format!("metadata: from_outgoing_context got an odd number of input pairs for metadata: %d", len(added)));
        }

        for i in (0..added.len()).step_by(2) {
            let key = added[i].to_lowercase();
            if let Some(current) = output.get_mut(&key) {
                *current.append(&mut added[i + 1]);
            }
        }
    }
    Some(output)
}

struct RawMetadata {
    metadata: Metadata,
    added: Vec<Vec<String>>
}
