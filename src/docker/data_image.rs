use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataImageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImageData>,
}

#[derive(Clone)]
pub struct DataImage(Rc<DataImage_>);

impl DataImage {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDocker) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image, including any tags or SHA256 repo digests."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_digest` after provisioning.\nThe image sha256 digest in the form of `repo[:tag]@sha256:<hash>`. It may be empty in the edge case where the local image was pulled from a repo, tagged locally, and then referred to in the data source by that local name/tag."]
    pub fn repo_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_digest", self.extract_ref()))
    }
}

impl Datasource for DataImage {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataImage {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataImage {
    type O = ListRef<DataImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataImage_ {
    fn extract_datasource_type(&self) -> String {
        "docker_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImage {
    pub tf_id: String,
    #[doc= "The name of the Docker image, including any tags or SHA256 repo digests."]
    pub name: PrimField<String>,
}

impl BuildDataImage {
    pub fn build(self, stack: &mut Stack) -> DataImage {
        let out = DataImage(Rc::new(DataImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImageRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImageRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image, including any tags or SHA256 repo digests."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_digest` after provisioning.\nThe image sha256 digest in the form of `repo[:tag]@sha256:<hash>`. It may be empty in the edge case where the local image was pulled from a repo, tagged locally, and then referred to in the data source by that local name/tag."]
    pub fn repo_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_digest", self.extract_ref()))
    }
}
