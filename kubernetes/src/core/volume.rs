//! - Concepts <https://kubernetes.io/docs/concepts/storage/volumes/>
//! - Reference <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume/>

use std::path::PathBuf;

use kfl::Decode;

use crate::{
    core::{FieldSelector, ResourceFieldSelector},
    quantity::Quantity,
};

/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
///
/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume/#Volume>
#[derive(Debug, Decode)]
pub struct Volume {
    /// Name of the volume. Must be a `DNS_LABEL` and unique within the pod.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names>
    #[kfl(argument)]
    name: String,

    // Exposed Persistent Volumes

    /// Represents a reference to a PersistentVolumeClaim in the same namespace.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#persistentvolumeclaims>
    persistent_volume_claim: Option<PersistentVolumeClaim>,

    // Projections
    // <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume/#projections>

    /// Represents a ConfigMap that should populate this volume
    config_map: Option<ConfigMap>,
    /// Represents a secret that should populate this volume.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes/#secret>
    secret: Option<Secret>,
    /// Represents downward API about the pod that should populate this volume.
    downward_api: Option<DownwardApi>,
    projected: Option<Projected>,

    // Local / Temporary Directory
    // <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume/#local-temporary-directory>

    /// Represents a temporary directory that shares a pod's lifetime.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes/#emptydir>
    empty_dir: Option<EmptyDir>,
    /// Represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes/#hostpath>
    host_path: Option<HostPath>,

    // Persistent Volumes
    // <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume/#persistent-volumes>
    aws_elastic_block_store: Option<AWSElasticBlockStore>
}

/// PersistentVolumeClaim references the user's PVC in the same namespace. This volume finds the bound PV and mounts that volume for the pod. A PersistentVolumeClaim is, essentially, a wrapper around another type of volume that is owned by someone else (the system).
#[derive(Debug, Decode)]
pub struct PersistentVolumeClaim {
    claim_name: String,
    read_only: Option<bool>
}

#[derive(Debug, Decode)]
pub struct ConfigMap {
    #[kfl(argument)]
    name: Option<String>,
    /// Specifies whether the ConfigMap or its keys must be defined.
    optional: Option<bool>,
    /// Mode bits used to set permissions on created files by default. Must be an octal value between `0000` and `0777` or a decimal value between `0` and `511`. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to `0644`. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    default_mode: Option<u16>,
    /// items if unspecified, each key-value pair in the [`data`][crate::core::config_map::ConfigMap::data] field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the `..` path or start with `..`.
    items: Vec<KeyToPath>
}

/// *Adapts a Secret into a volume.
/// 
/// The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling.*
#[derive(Debug, Decode)]
pub struct Secret {
    /// Name of the secret in the pod's namespace. More info: <https://kubernetes.io/docs/concepts/storage/volumes#secret>
    #[kfl(argument)]
    name: Option<String>,
    /// Whether the Secret or its keys must be defined.
    optional: Option<bool>,
    /// Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[kfl(property, default = 0o0644)]
    default_mode: u16,
    /// items If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    items: Vec<KeyToPath>
}

#[derive(Debug, Decode)]
pub struct DownwardApi {
    default_mode: Option<i32>,
    items: Vec<DownwardAPIVolumeFile>
}

#[derive(Debug, Decode)]
pub struct Projected {
    default_mode: i32,
    sources: Vec<projected::Source>
}

pub mod projected {
    use kfl::Decode;
    use super::KeyToPath;

    #[derive(Debug, Decode)]
    pub struct Source {
        config_map: ConfigMap
    }
    
    #[derive(Debug, Decode)]
    pub struct ConfigMap {
        name: String,
        optional: bool,
        items: Vec<KeyToPath>
    }
}

#[derive(Debug, Decode)]
pub struct EmptyDir {
    medium: Option<String>,
    size_limit: Option<Quantity>
}

/// <https://kubernetes.io/docs/concepts/storage/volumes/#hostpath>
#[derive(Debug, Decode)]
pub struct HostPath {
    path: PathBuf,
    r#type: Option<host_path::Type>
}

pub mod host_path {
    use kfl::DecodeScalar;

    #[derive(Debug, DecodeScalar, Default)]
    pub enum Type {
        #[default]
        Empty,
        DirectoryOrCreate,
        Directory,
        FileOrCreate,
        File,
        Socket,
        CharDevice,
        BlockDevice
    }
}

#[derive(Debug, Decode)]
pub struct AWSElasticBlockStore {
    volume_id: String,
    fs_type: Option<String>,
    partition: Option<String>,
    read_only: Option<bool>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume/#DownwardAPIVolumeFile>
#[derive(Debug, Decode)]
pub struct DownwardAPIVolumeFile {
    path: String,
    field_ref: Option<FieldSelector>,
    mode: Option<i32>,
    resource_field_ref: Option<ResourceFieldSelector>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/volume/#KeyToPath>
#[derive(Debug, Decode)]
pub struct KeyToPath {
    key: String,
    path: String,
    mode: Option<i32>
}
