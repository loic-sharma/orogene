use std::sync::Arc;

use rogga::PackageRequest;

use crate::set_relation::SetRelation;

#[derive(Clone)]
pub struct Term {
    pub positive: bool,
    pub root: bool,
    pub request: Arc<PackageRequest>,
}

impl Term {
    pub fn new(request: Arc<PackageRequest>, positive: bool, root: bool) -> Self {
        Self {
            root,
            positive,
            request,
        }
    }

    pub fn invert(&self) -> Self {
        Self::new(self.request.clone(), !self.positive, self.root)
    }

    pub fn relation(&self, _other: &Term) -> SetRelation {
        todo!()
    }
}
