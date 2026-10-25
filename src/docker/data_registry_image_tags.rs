#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataRegistryImageTagsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_skip_verify: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_semver: std::option::Option<terrars::PrimField<bool>>,
}

struct DataRegistryImageTags_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRegistryImageTagsData_>,
}

#[derive(Clone)]
pub struct DataRegistryImageTags(std::rc::Rc<DataRegistryImageTags_>);

impl DataRegistryImageTags {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDocker) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc =
        "Set the field `insecure_skip_verify`.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`."]
    pub fn set_insecure_skip_verify(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().insecure_skip_verify = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strict_semver`.\nIf `true`, only stable semantic version tags are returned. Prerelease tags such as `1.2.3-rc.1` are excluded as well as any other tags that do not conform to the semantic versioning specification. Defaults to `false`."]
    pub fn set_strict_semver(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().strict_semver = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this data source."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `insecure_skip_verify` after provisioning.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`."]
    pub fn insecure_skip_verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_skip_verify", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image repository, including any tag or digest. For example, `alpine:latest`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `strict_semver` after provisioning.\nIf `true`, only stable semantic version tags are returned. Prerelease tags such as `1.2.3-rc.1` are excluded as well as any other tags that do not conform to the semantic versioning specification. Defaults to `false`."]
    pub fn strict_semver(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_semver", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nList of available Docker image tags matching the specified criteria."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataRegistryImageTags {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRegistryImageTags { }

impl terrars::ToListMappable for DataRegistryImageTags {
    type O = terrars::ListRef<DataRegistryImageTagsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRegistryImageTags_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "docker_registry_image_tags".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRegistryImageTags {
    pub tf_id: std::string::String,
    #[doc = "The name of the Docker image repository, including any tag or digest. For example, `alpine:latest`."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataRegistryImageTags {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRegistryImageTags {
        let out = DataRegistryImageTags(std::rc::Rc::new(DataRegistryImageTags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRegistryImageTagsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                insecure_skip_verify: core::default::Default::default(),
                name: self.name,
                strict_semver: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRegistryImageTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRegistryImageTagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRegistryImageTagsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this data source."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `insecure_skip_verify` after provisioning.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`."]
    pub fn insecure_skip_verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_skip_verify", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image repository, including any tag or digest. For example, `alpine:latest`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `strict_semver` after provisioning.\nIf `true`, only stable semantic version tags are returned. Prerelease tags such as `1.2.3-rc.1` are excluded as well as any other tags that do not conform to the semantic versioning specification. Defaults to `false`."]
    pub fn strict_semver(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_semver", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nList of available Docker image tags matching the specified criteria."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}
