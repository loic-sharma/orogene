use std::str::FromStr;

mod parser;
mod types;

pub use types::{PackageSpec, PackageArgError, VersionReq};

impl PackageSpec {
    // TODO: Add where :\
    pub fn from_string<S: AsRef<str>>(s: S) -> Result<PackageSpec, PackageArgError> {
        parser::parse_package_spec(&s.as_ref())
    }

    pub fn resolve<N, S>(name: N, spec: S) -> Result<PackageSpec, PackageArgError>
    where
        N: AsRef<str>,
        S: AsRef<str>,
    {
        let mut arg = String::new();
        arg.push_str(name.as_ref());
        arg.push_str("@");
        arg.push_str(spec.as_ref());
        parser::parse_package_spec(&arg)
    }

    pub fn validate_name<S: AsRef<str>>(name: S) -> Result<String, PackageArgError> {
        let name = name.as_ref();
        Ok(name.into())
    }

    pub fn is_registry(&self) -> bool {
        match self {
            PackageSpec::Alias { package, .. } => package.is_registry(),
            PackageSpec::Dir { .. } => false,
            PackageSpec::Npm { .. } => true,
        }
    }
}

impl FromStr for PackageSpec {
    type Err = PackageArgError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        PackageSpec::from_string(s)
    }
}
