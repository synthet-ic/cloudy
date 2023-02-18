/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/metadata/v3/metadata.proto>
*/

/**
MetadataKey provides a general interface using `key` and `path` to retrieve value from [`Metadata`][crate::config::core::base::Metadata].

For example, for the following Metadata:

```yaml
filter-metadata:
  envoy.xxx:
    prop:
      foo: bar
      xyz:
        hello: envoy
```

The following MetadataKey will retrieve a string value "bar" from the Metadata.

```yaml
key: envoy.xxx
path:
- key: prop
- key: foo
```
*/
pub struct MetadataKey {
    /**
    The key name of Metadata to retrieve the Struct from the metadata.
    Typically, it represents a builtin subsystem or custom extension.

    [!key.is_empty()]
    */
    key: String,

    /**
    The path to retrieve the Value from the Struct. It can be a prefix or a full path, e.g. `[prop, xyz]` for a struct or `[prop, foo]` for a string in the example, which depends on the particular scenario.

    Note: Due to that only the key type segment is supported, the path can not specify a list unless the list is the last segment.

    [!path.is_empty()]
    */
    path: Vec<PathSegment>,
}

/**
Specifies the segment in a path to retrieve value from Metadata.
Currently it is only supported to specify the key, i.e. field name, as one segment of a path.
*/
pub struct PathSegment {
    segment: Segment
}

pub enum Segment {
    // option (validate.required) = true;

    /// If specified, use the key to retrieve the value in a Struct.
    /// [!.is_empty()]
    Key(String)
}

// Describes what kind of metadata.
pub struct MetadataKind {
    kind: Kind
}

pub enum Kind {
    // option (validate.required) = true;
    Request(Request),
    // 
    Route(Route),
    //
    Cluster(Cluster),
    //
    Host(Host)
}

/// Represents dynamic metadata associated with the request.
pub struct Request {
}

/// Represents metadata from [the route][crate::config::route::route_components::Route.metadata].
pub struct Route {
}

/// Represents metadata from [the upstream cluster][crate::config::cluster::cluster::Cluster.metadata].
pub struct Cluster {
}

/// Represents metadata from [the upstream host][crate::config::endpoint::endpoint_components::LBEndpoint::metadata].
pub struct Host {
}
