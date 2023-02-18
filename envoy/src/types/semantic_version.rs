/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/type/semantic_version.proto>
*/

/**
Envoy uses SemVer (<https://semver.org/>). Major/minor versions indicate expected behaviours and APIs, the patch version field is used only for security fixes and can be generally ignored.
*/
pub struct SemanticVersion {
    major_number: u32,

    minor_number: u32,

    patch: u32,
}
