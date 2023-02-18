/*!
<https://github.com/opencontainers/runtime-spec>
*/

use std::{
    collections::HashMap,
    path::PathBuf
};

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Runtime {
    oci_version: String,
    root: Option<Root>,
    mounts: Option<Vec<Mount>>,
    process: Option<Process>,
    #[serde(rename(serialize = "hostname"))]
    host_name: Option<String>,
    #[serde(rename(serialize = "domainname"))]
    domain_name: Option<String>,
    #[cfg(linux)]
    namespaces: Vec<Namespace>,
    #[cfg(linux)]
    devices: Option<Vec<Device>>,
    #[cfg(linux)]
    control_groups: ControlGroups,
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config.md#root>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Root {
    path: PathBuf,
    #[serde(rename(serialize = "readonly"))]
    read_only: Option<bool>   
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config.md#mounts>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Mount {
    destination: String,
    source: Option<String>,
    options: Option<Vec<MountOption>>,
    #[cfg(posix)]
    r#type: Option<MountType>,
    #[cfg(posix)]
    uid_mappings: Option<Vec<LinuxIDMapping>>,
    #[cfg(posix)]
    gid_mappings: Option<Vec<LinuxIDMapping>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "kebab-case"))]
pub enum MountOption {
    // Filesystem-Independent
    // https://man7.org/linux/man-pages/man8/mount.8.html#FILESYSTEM-INDEPENDENT_MOUNT_OPTIONS
    Async,
    Atime,
    NoAtime,
    Auto,
    NoAuto,
    Context(String),
    Defaults,
    Dev,
    NoDev,
    DirAtime,
    NoDirAtime,
    DirSync,
    Exec,
    NoExec,
    Group,
    Iversion,
    NoIversion,
    // #[serde(rename(serialize = "mand"))]
    Mand,
    NoMand,
    #[serde(rename(serialize = "_netdev"))]
    NetDev,
    NoFail,
    RelAtime,
    NoRelAtime,
    StrictAtime,
    LazyTime,
    NoLazyTime,
    Suid,
    NoSuid,
    Silent,
    Loud,
    Owner,
    Remount,
    #[serde(rename(serialize = "ro"))]
    ReadOnly,
    #[serde(rename(serialize = "rw"))]
    ReadWrite,
    Sync,
    User,
    NoUser,
    Users,
    Nosymfollow
}

pub type MountType = String;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Process {
    terminal: Option<bool>,
    console_size: Option<ConsoleSize>,
    cwd: PathBuf,
    env: Option<Vec<String>>,
    args: Option<Vec<String>>,
    command_line: Option<String>,
    #[cfg(posix)]
    #[serde(rename(serialize = "rlimits"))]
    resource_limits: Option<Vec<ResourceLimit>>,
    #[cfg(linux)]
    apparmor_profile: Option<String>,
    #[cfg(linux)]
    capabilities: Option<Capabilities>,
    #[cfg(linux)]
    no_new_privileges: Option<bool>,
    #[cfg(linux)]
    oom_score_adj: Option<i32>,
    #[cfg(linux)]
    selinux_label: Option<String>,
    #[cfg(posix)]
    user: User,
    #[cfg(posix)]
    hooks: Option<Hooks>,
    annotations: Option<HashMap<String, String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleSize {
    height: i32,
    width: i32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Capabilities {
    
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config.md#user>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct User {
    uid: i32,
    gid: i32,
    umask: Option<i32>,
    additional_gids: Option<Vec<i32>>
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md#namespaces>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Namespace {
    r#type: NamespaceType,
    path: Option<PathBuf>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub enum NamespaceType {
    Pid,
    Network,
    Mount,
    Ipc,
    Uts,
    User,
    Cgroup
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md#devices>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Device {
    r#type: DeviceType,
    path: PathBuf,
    major: Option<i64>,
    minor: Option<i64>,
    file_mode: Option<u32>,
    uid: Option<u32>,
    gid: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum DeviceType {
    #[serde(rename(serialize = "c"))]
    Character,
    #[serde(rename(serialize = "u"))]
    CharacterUnbuffered,
    #[serde(rename(serialize = "b"))]
    Block,
    #[serde(rename(serialize = "p"))]
    Fifo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct ControlGroups {
    #[serde(rename(serialize = "cgroupsPath"))]
    path: Option<PathBuf>,
    resources: Option<Resources>,
    devices: Option<Vec<ControlGroupsDevice>>,
    memory: Option<Memory>,
    cpu: Option<CPU>,
    #[serde(rename(serialize = "blockIO"))]
    block_io: Option<BlockIO>,
    #[serde(rename(serialize = "hugepageLimits"))]
    huge_page_limits: Option<Vec<HugePageLimits>>,
    network: Option<Network>,
    pids: Option<Pids>,
    rdma: Option<RDMA>,
    unified: Option<Unified>,
    // intel_rdt: Option<IntelRdt>,
    sysctl: Option<Sysctl>,
    seccomp: Option<Seccomp>,
    rootfs_propagation: Option<RootfsPropagation>,
    masked_paths: Option<Vec<String>>,
    #[serde(rename(serialize = "readonlyPaths"))]
    read_only_paths: Option<Vec<String>>,
    mount_label: Option<String>,
    personality: Option<Personality>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Resources {

}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct ControlGroupsDevice {
    allow: bool,
    r#type: Option<ControlGroupsDeviceType>,
    major: Option<i64>,
    minor: Option<i64>,
    access: Option<Vec<ControlGroupsDeviceAccess>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum ControlGroupsDeviceType {
    #[serde(rename(serialize = "a"))]
    All,
    #[serde(rename(serialize = "c"))]
    Char,
    #[serde(rename(serialize = "b"))]
    Block
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub enum ControlGroupsDeviceAccess {
    #[serde(rename(serialize = "r"))]
    Read,
    #[serde(rename(serialize = "w"))]
    Write,
    #[serde(rename(serialize = "m"))]
    Mknod
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Memory {
    limit: Option<i64>,
    reservation: Option<i64>,
    swap: Option<i64>,
    /// NOT RECOMMENDED
    kernel: Option<i64>,
    /// NOT RECOMMENDED
    #[serde(rename(serialize = "kernelTCP"))]
    kernel_tcp: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct CPU {
    shares: Option<u64>,
    quota: Option<i64>,
    period: Option<u64>,
    realtime_runtime: Option<i64>,
    realtime_period: Option<u64>,
    cpus: Option<String>,
    #[serde(rename(serialize = "mems"))]
    memory_nodes: Option<String>,
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md#block-io>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct BlockIO {

}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct HugePageLimits {
    page_size: String,
    limit: u64
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Network {
    #[serde(rename(serialize = "classID"))]
    class_id: Option<u32>,
    priorities: Option<Vec<Priority>>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Priority {
    name: String,
    priority: u32
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md#pids>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Pids {
    limit: i64
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md#rdma>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct RDMA {
    hca_handles: Option<u32>,
    hca_objects: Option<u32>
}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md#unified>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Unified {

}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md#sysctl>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Sysctl {

}

/// <https://github.com/opencontainers/runtime-spec/blob/main/config-linux.md#seccomp>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Seccomp {
    default_action: String,
    default_errno_ret: Option<u32>,
    architectures: Option<Vec<Architecture>>,
    flags: Option<Vec<SeccompFlag>>,
    listener_path: Option<String>,
    listener_metadata: Option<String>,
    syscalls: Option<Vec<Syscall>>
}

#[derive(Debug, Serialize, Deserialize)]
// TOOD serialize = SCMP_ARCH_*
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE",
                   deserialize = "lowercase"))]
pub enum Architecture {
    X86,
    X86_64,
    X32,
    Arm,
    Aarch64,
    Mips,
    Mips64,
    Mips64n32,
    Mipsel,
    Mipsel64,
    Mipsel64n32,
    Ppc,
    Ppc64,
    Ppc64le,
    S390,
    S390x,
    Parisc,
    Parisc64,
    Riscv64
}

#[derive(Debug, Serialize, Deserialize)]
// TODO serialize = SECCOMP_FILTER_FLAG_*
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE",
                   deserialize = "lowercase"))]
pub enum SeccompFlag {
    Tsync,
    Log,
    SpecAllow,
    WaitKillableRecv
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Syscall {
    names: Vec<String>,
    action: SyscallAction,
    errno_ret: Option<u32>,
    args: Option<Vec<SyscallArg>>
}

#[derive(Debug, Serialize, Deserialize)]
// TODO serialize = SCMP_ACT_*
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "kebab-case"))]
pub enum SyscallAction {
    Kill,
    KillProcess,
    KillThread,
    Trap,
    Errno,
    Trace,
    Allow,
    Log,
    Notify
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct SyscallArg {
    index: u32,
    value: u64,
    value_two: Option<u64>,
    #[serde(rename(serialize = "op"))]
    operator: SyscallArgOperator
}

#[derive(Debug, Serialize, Deserialize)]
// TODO serialize = SCMP_CMP_*
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE",
                   deserialize = "kebab-case"))]
pub enum SyscallArgOperator {
    Ne,
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
    MaskedEq
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct ContainerProcessState {

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "kebab-case"))]
pub enum RootfsPropagation {
    Shared,
    Slave,
    Private,
    Unbindable
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Personality {
    domain: ExecutionDomain,
    flags: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE",
                   deserialize = "kebab-case"))]
pub enum ExecutionDomain {
    Linux,
    Linux32
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Hooks {
    create_runtime: Option<Vec<Hook>>,
    create_container: Option<Vec<Hook>>,
    start_container: Option<Vec<Hook>>,
    poststart: Option<Vec<Hook>>,
    poststop: Option<Vec<Hook>>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
pub struct Hook {

}
