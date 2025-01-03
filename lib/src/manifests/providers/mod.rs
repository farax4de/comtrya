mod local;
use local::LocalManifestProvider;
use std::path::PathBuf;
mod git;
use git::GitManifestProvider;

pub fn register_providers() -> Vec<Box<dyn ManifestProvider>> {
    vec![
        Box::new(LocalManifestProvider),
        Box::new(GitManifestProvider),
    ]
}

#[derive(Debug, PartialEq, Eq)]
pub enum ManifestProviderError {
    NoResolution,
}

/// ManifestProviders are responsible for taking a String
/// and returning a `PathBuf`. Providers, such as Git, are
/// responsible for accepting the String and cloning the
/// repository, in-order to return the `PathBuf`.
pub trait ManifestProvider {
    /// This functions is called to establish if it could potentially
    /// be able to resolve the url provided
    fn looks_familiar(&self, url: &str) -> bool;

    /// This function is responsible for returning a PathBuf with
    /// the directory containing the manifests
    fn resolve(&self, url: &str) -> Result<PathBuf, ManifestProviderError>;
}
