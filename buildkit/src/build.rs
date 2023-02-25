pub struct Build {
    import_cache: Option<ImportCache>
}

pub struct ImportCache {
    /// specify image name(s)
    name: Value,
    /// push after creating the image
    push: bool,
    /// push unnamed image
    push_by_digest: bool,
    /// push to insecure HTTP registry
    registry_insecure: bool,
    /// use OCI mediatypes in configuration JSON instead of Docker's
    oci_mediatypes: bool,
    /// unpack image after creation (for use with containerd)
    unpack: bool,
    /// name image with prefix@<digest>, used for anonymous images
    dangling_name_prefix: Value,
    /// add additional canonical name name@<digest>
    name_canonical: bool,
    /// choose compression type for layers newly created and cached, gzip is default value. estargz should be used with oci-mediatypes=true.
    compression: Compression,
    /// compression level for gzip, estargz (0-9) and zstd (0-22)
    compression_level: Value,
    /// forcefully apply compression option to all layers (including already existing layers)
    force_compression: bool,
    /// attach inline build info in image config (default true)
    build_info: bool,
    /// attach inline build info attributes in image config (default false)
    build_info_attrs: bool,
    /// store the result images to the worker's (e.g. containerd) image store as well as ensures that the image has all blobs in the content store (default true). Ignored if the worker doesn't have image store (e.g. OCI worker).
    store: bool,
    annotation: HashMap<String, String>
}
      
pub enum Compression {
    Uncompressed,
    Gzip,
    Estargz,
    Zstd
}
