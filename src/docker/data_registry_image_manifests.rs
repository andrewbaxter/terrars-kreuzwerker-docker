#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataRegistryImageManifestsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_skip_verify: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<std::vec::Vec<DataRegistryImageManifests_AuthConfig>>,
    dynamic: DataRegistryImageManifestsDynamic,
}

struct DataRegistryImageManifests_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRegistryImageManifestsData_>,
}

#[derive(Clone)]
pub struct DataRegistryImageManifests(std::rc::Rc<DataRegistryImageManifests_>);

impl DataRegistryImageManifests {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `insecure_skip_verify`.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`"]
    pub fn set_insecure_skip_verify(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().insecure_skip_verify = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataRegistryImageManifests_AuthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `insecure_skip_verify` after provisioning.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`"]
    pub fn insecure_skip_verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_skip_verify", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `manifests` after provisioning.\nThe metadata for each manifest in the image"]
    pub fn manifests(&self) -> terrars::SetRef<DataRegistryImageManifests_ManifestsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.manifests", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image, including any tags. e.g. `alpine:latest`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<DataRegistryImageManifests_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataRegistryImageManifests {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRegistryImageManifests { }

impl terrars::ToListMappable for DataRegistryImageManifests {
    type O = terrars::ListRef<DataRegistryImageManifestsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRegistryImageManifests_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "docker_registry_image_manifests".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRegistryImageManifests {
    pub tf_id: std::string::String,
    #[doc = "The name of the Docker image, including any tags. e.g. `alpine:latest`"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataRegistryImageManifests {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRegistryImageManifests {
        let out = DataRegistryImageManifests(std::rc::Rc::new(DataRegistryImageManifests_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRegistryImageManifestsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                insecure_skip_verify: core::default::Default::default(),
                name: self.name,
                auth_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRegistryImageManifestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRegistryImageManifestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRegistryImageManifestsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `insecure_skip_verify` after provisioning.\nIf `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`"]
    pub fn insecure_skip_verify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_skip_verify", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `manifests` after provisioning.\nThe metadata for each manifest in the image"]
    pub fn manifests(&self) -> terrars::SetRef<DataRegistryImageManifests_ManifestsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.manifests", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image, including any tags. e.g. `alpine:latest`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<DataRegistryImageManifests_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataRegistryImageManifestsDynamic {
    auth_config: std::option::Option<terrars::DynamicBlock<DataRegistryImageManifests_AuthConfig>>,
}

#[derive(Serialize)]
pub struct DataRegistryImageManifests_Manifests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_digest: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRegistryImageManifests_Manifests {
    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `media_type`.\n"]
    pub fn set_media_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.media_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os`.\n"]
    pub fn set_os(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sha256_digest`.\n"]
    pub fn set_sha256_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_digest = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRegistryImageManifests_Manifests {
    type O = terrars::BlockAssignable<DataRegistryImageManifests_Manifests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRegistryImageManifests_Manifests {}

impl BuildDataRegistryImageManifests_Manifests {
    pub fn build(self) -> DataRegistryImageManifests_Manifests {
        DataRegistryImageManifests_Manifests {
            architecture: core::default::Default::default(),
            media_type: core::default::Default::default(),
            os: core::default::Default::default(),
            sha256_digest: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRegistryImageManifests_ManifestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRegistryImageManifests_ManifestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRegistryImageManifests_ManifestsRef {
        DataRegistryImageManifests_ManifestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegistryImageManifests_ManifestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `media_type` after provisioning.\n"]
    pub fn media_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.media_type", self.base))
    }

    #[doc = "Get a reference to the value of field `os` after provisioning.\n"]
    pub fn os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os", self.base))
    }

    #[doc = "Get a reference to the value of field `sha256_digest` after provisioning.\n"]
    pub fn sha256_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_digest", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRegistryImageManifests_AuthConfig {
    address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRegistryImageManifests_AuthConfig {
    #[doc = "Set the field `password`.\nThe password for the Docker registry."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username for the Docker registry."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRegistryImageManifests_AuthConfig {
    type O = terrars::BlockAssignable<DataRegistryImageManifests_AuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRegistryImageManifests_AuthConfig {
    #[doc = "The address of the Docker registry."]
    pub address: terrars::PrimField<std::string::String>,
}

impl BuildDataRegistryImageManifests_AuthConfig {
    pub fn build(self) -> DataRegistryImageManifests_AuthConfig {
        DataRegistryImageManifests_AuthConfig {
            address: self.address,
            password: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRegistryImageManifests_AuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRegistryImageManifests_AuthConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRegistryImageManifests_AuthConfigRef {
        DataRegistryImageManifests_AuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegistryImageManifests_AuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe address of the Docker registry."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nThe password for the Docker registry."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username for the Docker registry."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
