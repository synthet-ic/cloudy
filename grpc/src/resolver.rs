/*!
<https://github.com/grpc/grpc-go/blob/master/resolver/resolver.go>
*/

pub mod manual;

use std::{
    default::Default,
    net::TcpStream,
    task::Context
};

use crate::{
    attributes::Attributes,
    credentials::{Bundle, TransportCredentials},
    internal::pretty,
    service_config::ParseResult
};

pub struct Resolver {
    /// Map from scheme to resolver builder.
    map: HashMap<String, Builder>,

    /// Default scheme to use.
    default_scheme: String = "passthrough";
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
        }
    }
}

// TODO(bar) install dns resolver in init(){}.

impl Resolver {
    /**
    Registers the resolver builder to the resolver map. b.Scheme will be used as the scheme registered with this builder.

    NOTE: this function must only be called during initialisation time (i.e. in an init() function), and is not thread-safe. If multiple Resolvers are registered with the same name, the one registered last will take effect.
    */
    pub fn register(&mut self, builder: impl Builder) {
        self.map.insert(builder.scheme(), builder);
    }

    /**
    Returns the resolver builder registered with the given scheme.

    If no builder is register with the scheme, nil will be returned.
    */
    pub fn get(&self, scheme: String) -> Option<Builder> {
         map.get(scheme)
    }

    /**
    Sets the default scheme that will be used. The default default scheme is "passthrough".

    NOTE: this function must only be called during initialisation time (i.e. in an init() function), and is not thread-safe. The scheme set last overrides previously set values.
    */
    pub fn set_default_scheme(&mut self, scheme: String) {
        self.default_scheme = scheme;
    }

    /// Gets the default scheme that will be used.
    pub fn get_default_scheme(&self) -> String {
        self.default_scheme
    }
}

/**
Address represents a server the client connects to.

# Experimental

Notice: This type is EXPERIMENTAL and may be changed or removed in a later release.
*/
pub struct Address {
    /// Server address on which a connection will be established.
    pub address: String,

    /**
    Name of this address.
    If non-empty, the server_name is used as the transport certification authority for the address, instead of the hostname from the Dial target string. In most cases, this should not be set.

    If Type is GRPCLB, server_name should be the name of the remote load balancer, not the name of the backend.

    WARNING: server_name must only be populated with trusted values. It is insecure to populate it with data from untrusted inputs since untrusted values could be used to bypass the authority checks performed by TLS.
    */
    pub server_name: String,

    /// Arbitrary data about this address intended for consumption by the SubConn.
    pub attributes: Option<Attributes>,

    /// Arbitrary data about this address intended for consumption by the LB policy. These attribes do not affect SubConn creation, connection establishment, handshaking, etc.
    pub balancer_attributes: Option<Attributes>,
}

impl Address {
    /**
    Equal returns whether a and o are identical. Metadata is compared directly, not with any recursive introspection.
    */
    pub fn equal(&self, other: Address) -> bool {
          self.addr == other.addr
        && self.server_name == other.server_name
        && self.attributes.Equal(other.attributes)
        && self.balancer_attributes.Equal(other.balancer_attributes)
    }

    /// String returns JSON formatted string representation of the address.
    pub fn string(&self) -> String {
        return pretty.ToJSON(self)
    }
}

/// BuildOptions includes additional information for the builder to create the resolver.
pub struct BuildOptions {
    /// Whether a resolver implementation should fetch service config data.
    pub disable_service_config: bool,

    /// Transport credentials used by the ClientConn for communicating with the target gRPC service (set via WithTransportCredentials). In cases where a name resolution service requires the same credentials, the resolver may use this field. In most cases though, it is not appropriate, and this field may be ignored.
    pub dial_creds: TransportCredentials,

    /// Credentials bundle used by the ClientConn for communicating with the target gRPC service (set via WithCredentialsBundle). In cases where a name resolution service requires the same credentials, the resolver may use this field. In most cases though, it is not appropriate, and this field may be ignored.
    pub creds_bundle: Bundle,

    /// Custom dialer used by the ClientConn for dialling the target gRPC service (set via WithDialer). In cases where a name resolution service requires the same dialer, the resolver may use this field. In most cases though, it is not appropriate, and this field may be ignored.
    pub dialer: fn(Context, String) -> Result<TcpStream>
}

/// State contains the current `Resolver` state relevant to the ClientConn.
pub struct State {
    /// The latest set of resolved addresses for the target.
    pub addresses: Vec<Address>,

    /// Result from parsing the latest service config. If it is nil, it indicates no service config is present or the resolver does not provide service configs.
    pub service_config: Option<ParseResult>,

    /// Arbitrary data about the resolver intended for consumption by the load balancing policy.
    pub attributes: Option<Attributes>
}

/**
ClientConn contains the callbacks for resolver to notify any updates to the gRPC ClientConn.

This interface is to be implemented by gRPC. Users should not need a brand new implementation of this interface. For the situations like testing, the new implementation should embed this interface. This allows gRPC to add new methods to this interface.
*/
pub trait ClientConn {
    /// Updates the state of the ClientConn appropriately.
    fn update_state(State) -> error;

    /**
    Notifies the ClientConn that the Resolver encountered an error.  The ClientConn will notify the load balancer and begin calling ResolveNow on the Resolver with exponential backoff.
    */
    fn report_error(error);

    /// Parses the provided service config and returns an object that provides the parsed config.
    fn parse_service_config(serviceConfigJSON: String) -> Option<ParseResult>;
}

/**
Target represents a target for gRPC, as specified in: <https://github.com/grpc/grpc/blob/master/doc/naming.md>.
It is parsed from the target string that gets passed into Dial or DialContext by the user. And gRPC passes it to the resolver and the balancer.

If the target follows the naming spec, and the parsed scheme is registered with gRPC, we will parse the target string according to the spec. If the target does not contain a scheme or if the parsed scheme is not registered (i.e. no corresponding resolver available to resolve the endpoint), we will apply the default scheme, and will attempt to reparse it.

Examples:
- "dns://some_authority/foo.bar"
Target{Scheme: "dns", Authority: "some_authority", Endpoint: "foo.bar"}
- "foo.bar"
Target{Scheme: resolver.get_default_scheme(), Endpoint: "foo.bar"}
- "unknown_scheme://authority/endpoint"
Target{Scheme: resolver.get_default_scheme(), Endpoint: "unknown_scheme://authority/endpoint"}
*/
pub struct Target {
    /**
    URL contains the parsed dial target with an optional default scheme added to it if the original dial target contained no scheme or contained an unregistered scheme. Any query params specified in the original dial target can be accessed from here.
    */
    pub url: url.URL
}

/// Builder creates a resolver that will be used to watch name resolution updates.
pub trait Builder {
    /**
    Build creates a new resolver for the given target.

    gRPC dial calls Build synchronously, and fails if the returned error is not nil.
    */
    fn build(target: Target, cc: ClientConn, opts: BuildOptions) -> Result<Resolver>;

    /**
    Returns the scheme supported by this resolver.
    Scheme is defined at <https://github.com/grpc/grpc/blob/master/doc/naming.md>.
    */
    fn scheme() -> String;
}

/// ResolveNowOptions includes additional information for ResolveNow.
pub struct ResolveNowOptions;

/**
Resolver watches for the updates on the specified target.
Updates include address updates and service config updates.
*/
pub trait Resolver {
    /**
    Will be called by gRPC to try to resolve the target name again. It's just a hint, resolver can ignore this if it's not necessary.

    It could be called multiple times concurrently.
    */
    fn resolve_now();

    /// Closes the resolver.
    fn close();
}

/**
Removes the resolver builder with the given scheme from the resolver map.
This function is for testing only.
*/
pub fn unregister_for_testing(scheme: String) {
    delete(m, scheme)
}
