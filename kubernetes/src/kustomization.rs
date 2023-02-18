//! Reference <https://kubectl.docs.kubernetes.io/references/kustomize/kustomization/>

use std::path::PathBuf;

use kfl::Decode;

type PathOrUrl = PathBuf;

#[derive(Debug, Decode)]
pub struct Kustomization {
    resources: Vec<PathOrUrl>,
    generators: Vec<PathOrUrl>,
    transformers: Vec<PathOrUrl>,
    validators: Vec<PathOrUrl>,
    build_metadata: Vec<BuildMetadata>,
}

#[derive(Debug, Decode)]
pub enum BuildMetadata {
    ManagedByLabel,
    OriginAnnotations,
    TransformerAnnotations
}

#[derive(Debug, Decode)]
pub struct PrefixSuffixTransformer {
}
