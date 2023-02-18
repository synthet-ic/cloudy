/*!
<https://github.com/grpc/grpc-go/blob/master/encoding/encoding.go>
*/

use std::{
    collections::HashMap,
    io
};

use crate::internal::grpcutil;

/**
Identity specifies the optional encoding for uncompressed streams.
It is intended for grpc internal use only.
*/
const Identity = "identity";

/// Compressor is used for compressing and decompressing when sending or receiving messages.
pub trait Compressor {
    /**
    Writes the data written to wc to w after compressing it. If an error occurs while initialising the compressor, that error is returned instead.
    */
    fn compress(w: io.Writer) -> Result<io.WriteCloser>;

    /**
    Reads data from r, decompresses it, and provides the uncompressed data via the returned io.Reader. If an error occurs while initialising the decompressor, that error is returned instead.
    */
    fn decompress(r: io.Reader) -> Resutl<io.Reader>;

    /**
    Name of the compression codec and is used to set the content coding header. The result must be static; the result cannot change between calls.
    */
    fn name() -> String;

    // If a Compressor implements
    // DecompressedSize(compressedBytes Vec<u8>) int, gRPC will call it
    // to determine the size of the buffer allocated for the result of decompression.
    // Return -1 to indicate unknown size.
    //
    // Experimental
    //
    // Notice: This API is EXPERIMENTAL and may be changed or removed in a
    // later release.
}

var registeredCompressor = make(HashMap<String, Compressor>);

/**
Registers the compressor with gRPC by its name.  It can be activated when sending an RPC via grpc.UseCompressor().  It will be automatically accessed when receiving a message based on the content coding header.  Servers also use it to send a response with the same encoding as the request.

NOTE: this function must only be called during initialisation time (i.e. in
an init() function), and is not thread-safe.  If multiple Compressors are
registered with the same name, the one registered last will take effect.
*/
pub fn register_compressor(c: Compressor) {
    registeredCompressor[c.Name()] = c;
    grpcutil.RegisteredCompressorNames = append(grpcutil.RegisteredCompressorNames, c.Name());
}

/// Returns Compressor for the given compressor name.
pub fn get_compressor(name: String) -> Compressor {
    registeredCompressor[name]
}

/**
Codec defines the interface gRPC uses to encode and decode messages.  Note that implementations of this interface must be thread safe; a Codec's methods can be called from concurrent goroutines.
*/
pub trait Codec {
    /// Marshal returns the wire format of v.
    fn marshal(v: interface{}) -> Result<Vec<u8>>;

    /// Unmarshal parses the wire format into v.
    fn unmarshal(data: Vec<u8>, v: interface{}) -> Result<()>;

    /**
    Name returns the name of the Codec implementation. The returned string will be used as part of content type in transmission. The result must be static; the result cannot change between calls.
    */
    fn name() -> String;
}

var registeredCodecs = make(HashMap<string, Codec>);

/**
Registers the provided Codec for use with all gRPC clients and servers.

The Codec will be stored and looked up by result of its Name() method, which should match the content-subtype of the encoding handled by the Codec.  This is case-insensitive, and is stored and looked up as lowercase.  If the result of calling Name() is an empty string, register_codec will panic. See Content-Type on
<https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md#requests> for more details.

NOTE: this function must only be called during initialisation time (i.e. in an init() function), and is not thread-safe.  If multiple Codecs are registered with the same name, the one registered last will take effect.
*/
pub fn register_codec(codec: Codec) {
    if codec == nil {
        panic("cannot register a nil Codec")
    }
    if codec.name() == "" {
        panic("cannot register Codec with empty string result for Name()")
    }
    contentSubtype := strings.ToLower(codec.Name())
    registeredCodecs[contentSubtype] = codec
}

/**
Gets a registered Codec by content-subtype, or nil if no Codec is registered for the content-subtype.

The content-subtype is expected to be lowercase.
*/
pub fn get_codec(contentSubtype: String) -> Codec {
    registeredCodecs[contentSubtype]
}
