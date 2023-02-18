/*!
<https://github.com/grpc/grpc-go/blob/master/codec.go>
*/

use crate::encoding::{
    Codec,
    // To register the Codec for "proto"
    proto
};

/**
BaseCodec contains the functionality of both Codec and encoding.Codec, but omits the name/string, which vary between the two and are not needed for anything besides the registry in the encoding package.
*/
trait BaseCodec {
    fn marshal(v: interface{}) -> Result<Vec<u8>>;
    fn unmarshal(data: Vec<u8>, v: interface{}) -> Result<()>;
}

var _ baseCodec = Codec(nil);
