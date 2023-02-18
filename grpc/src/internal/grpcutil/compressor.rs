/*!
<https://github.com/grpc/grpc-go/blob/master/internal/grpcutil/compressor.go>
*/

use crate::internal::envconfig::AdvertiseCompressors;

/// RegisteredCompressorNames holds names of the registered compressors.
var RegisteredCompressorNames Vec<String>;

/// Returns true when name is available in registry.
pub fn is_compressor_name_registered(name: String) -> bool {
    for compressor in RegisteredCompressorNames.iter() {
        if compressor == name {
            return true
        }
    }
    false
}

/// registered_compressors returns a string of registered compressor names separated by comma.
pub fn registered_compressors() -> String {
    if !AdvertiseCompressors {
        return ""
    }
    return strings.Join(RegisteredCompressorNames, ",")
}
