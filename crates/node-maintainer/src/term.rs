use std::sync::Arc;

use rogga::PackageRequest;

use crate::set_relation::SetRelation;

#[derive(Clone)]
pub struct Term {
    pub positive: bool,
    pub request: Arc<Option<PackageRequest>>,
}

impl Term {
    pub fn new(request: Arc<Option<PackageRequest>>, positive: bool) -> Self {
        Self { positive, request }
    }

    pub fn invert(&self) -> Self {
        Self::new(self.request.clone(), !self.positive)
    }

    pub fn is_root(&self) -> bool {
        self.request.is_none()
    }

    pub fn relation(&self, _other: &Term) -> SetRelation {
        todo!()
    }
}
