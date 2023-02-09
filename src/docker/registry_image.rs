use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct RegistryImageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_skip_verify: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_remotely: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggers: Option<RecField<PrimField<String>>>,
}

struct RegistryImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RegistryImageData>,
}

#[derive(Clone)]
pub struct RegistryImage(Rc<RegistryImage_>);

impl RegistryImage {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDocker) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `insecure_skip_verify`.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`"]
    pub fn set_insecure_skip_verify(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().insecure_skip_verify = Some(v.into());
        self
    }

    #[doc= "Set the field `keep_remotely`.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`"]
    pub fn set_keep_remotely(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().keep_remotely = Some(v.into());
        self
    }

    #[doc= "Set the field `triggers`.\nA map of arbitrary strings that, when changed, will force the `docker_registry_image` resource to be replaced. This can be used to repush a local image"]
    pub fn set_triggers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().triggers = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `insecure_skip_verify` after provisioning.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`"]
    pub fn insecure_skip_verify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.insecure_skip_verify", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keep_remotely` after provisioning.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`"]
    pub fn keep_remotely(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_remotely", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha256_digest` after provisioning.\nThe sha256 digest of the image."]
    pub fn sha256_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256_digest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `triggers` after provisioning.\nA map of arbitrary strings that, when changed, will force the `docker_registry_image` resource to be replaced. This can be used to repush a local image"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}

impl Resource for RegistryImage {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for RegistryImage {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for RegistryImage {
    type O = ListRef<RegistryImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RegistryImage_ {
    fn extract_resource_type(&self) -> String {
        "docker_registry_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRegistryImage {
    pub tf_id: String,
    #[doc= "The name of the Docker image."]
    pub name: PrimField<String>,
}

impl BuildRegistryImage {
    pub fn build(self, stack: &mut Stack) -> RegistryImage {
        let out = RegistryImage(Rc::new(RegistryImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RegistryImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                insecure_skip_verify: core::default::Default::default(),
                keep_remotely: core::default::Default::default(),
                name: self.name,
                triggers: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RegistryImageRef {
    shared: StackShared,
    base: String,
}

impl Ref for RegistryImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RegistryImageRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `insecure_skip_verify` after provisioning.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`"]
    pub fn insecure_skip_verify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.insecure_skip_verify", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keep_remotely` after provisioning.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`"]
    pub fn keep_remotely(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_remotely", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha256_digest` after provisioning.\nThe sha256 digest of the image."]
    pub fn sha256_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256_digest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `triggers` after provisioning.\nA map of arbitrary strings that, when changed, will force the `docker_registry_image` resource to be replaced. This can be used to repush a local image"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}
