use std::collections::HashMap;
use std::path::Path;

use async_trait::async_trait;
use futures::io::AsyncRead;
use http_types::Url;
use oro_manifest::OroManifestBuilder;
use package_spec::PackageSpec;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Internal, Result};
use crate::fetch::PackageFetcher;
use crate::package::{Package, PackageRequest};
use crate::packument::{Dist, Packument, VersionMetadata};

use oro_node_semver::Version;

pub struct DirFetcher {
    name: Option<String>,
}

impl DirFetcher {
    pub fn new() -> Self {
        Self { name: None }
    }
}

impl DirFetcher {
    async fn packument_from_spec(&mut self, spec: &PackageSpec) -> Result<Packument> {
        let path = match spec {
            PackageSpec::Dir { path, from } => from.join(path),
            _ => panic!("There shouldn't be anything but Dirs here"),
        };
        // TODO: Orogene.toml?
        let json = async_std::fs::read_to_string(&path.join("package.json"))
            .await
            .to_internal()
            .with_context(|| "Failed to read package.json".into())?;
        let pkgjson: PkgJson = serde_json::from_str(&json)
            .to_internal()
            .with_context(|| "Failed to parse package.json".into())?;
        Ok(pkgjson.into_packument(&path)?)
    }
}

#[async_trait]
impl PackageFetcher for DirFetcher {
    async fn name(&mut self, spec: &PackageSpec) -> Result<String> {
        if let Some(ref name) = self.name {
            Ok(name.clone())
        } else if let PackageSpec::Dir { ref path, ref from } = spec {
            self.name = Some(
                self.packument_from_spec(spec)
                    .await?
                    .versions
                    .iter()
                    .next()
                    .unwrap()
                    .1
                    .manifest
                    .clone()
                    .name
                    .unwrap_or_else(|| {
                        let canon = from.join(path).canonicalize();
                        let path = canon.as_ref().map(|p| p.file_name());
                        if let Ok(Some(name)) = path {
                            name.to_string_lossy().into()
                        } else {
                            "".into()
                        }
                    }),
            );
            self.name
                .as_ref()
                .cloned()
                .ok_or_else(|| Error::MiscError("This is impossible".into()))
        } else {
            unreachable!()
        }
    }

    async fn manifest(&mut self, _pkg: &Package) -> Result<VersionMetadata> {
        unimplemented!()
    }

    async fn packument(&mut self, pkg: &PackageRequest) -> Result<Packument> {
        self.packument_from_spec(pkg.spec()).await
    }

    async fn tarball(
        &mut self,
        _pkg: &Package,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send + Sync>> {
        // TODO: need to implement pack before this can be implemented :(
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize)]
struct PkgJson {
    name: Option<String>,
    version: Option<Version>,
    description: Option<String>,
}

impl PkgJson {
    pub fn into_packument(self, path: impl AsRef<Path>) -> Result<Packument> {
        let PkgJson { name, version, .. } = self;
        let name = name.or_else(|| {
            if let Some(name) = path.as_ref().file_name() {
                Some(name.to_string_lossy().into())
            } else {
                None
            }
        }).ok_or_else(|| Error::MiscError("Failed to find a valid name. Make sure the package.json has a `name` field, or that it exists inside a named directory.".into()))?;
        let version = version.unwrap_or_else(|| Version::parse("0.0.0").expect("Oops, typo"));
        let mut packument = Packument {
            versions: HashMap::new(),
            time: HashMap::new(),
            tags: HashMap::new(),
            rest: HashMap::new(),
        };
        let manifest = OroManifestBuilder::default()
            .name(name)
            .version(version.clone())
            .build()
            .unwrap();
        let version_meta = VersionMetadata {
            dist: Dist {
                shasum: "".into(),
                tarball: Url::parse(&format!("file:{}", path.as_ref().display())).to_internal()?,

                integrity: None,
                file_count: None,
                unpacked_size: None,
                npm_signature: None,
                rest: HashMap::new(),
            },
            npm_user: None,
            has_shrinkwrap: None,
            maintainers: Vec::new(),
            deprecated: None,
            manifest,
        };
        packument.tags.insert("latest".into(), version.clone());
        packument.versions.insert(version, version_meta);
        Ok(packument)
    }
}
