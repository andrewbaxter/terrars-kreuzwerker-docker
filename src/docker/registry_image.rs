#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct RegistryImageData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_skip_verify: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_remotely: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<std::vec::Vec<RegistryImage_AuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build: std::option::Option<std::vec::Vec<RegistryImage_Build>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RegistryImage_Timeouts>,
    dynamic: RegistryImageDynamic,
}

struct RegistryImage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RegistryImageData_>,
}

#[derive(Clone)]
pub struct RegistryImage(std::rc::Rc<RegistryImage_>);

impl RegistryImage {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDocker) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc =
        "Set the field `keep_remotely`.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`"]
    pub fn set_keep_remotely(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().keep_remotely = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `triggers`.\nA map of arbitrary strings that, when changed, will force the `docker_registry_image` resource to be replaced. This can be used to repush a local image"]
    pub fn set_triggers(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().triggers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RegistryImage_AuthConfig>>,
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

    #[doc = "Set the field `build`.\n"]
    pub fn set_build(self, v: impl std::convert::Into<terrars::BlockAssignable<RegistryImage_Build>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<RegistryImage_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `keep_remotely` after provisioning.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`"]
    pub fn keep_remotely(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_remotely", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sha256_digest` after provisioning.\nThe sha256 digest of the image."]
    pub fn sha256_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_digest", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `triggers` after provisioning.\nA map of arbitrary strings that, when changed, will force the `docker_registry_image` resource to be replaced. This can be used to repush a local image"]
    pub fn triggers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<RegistryImage_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RegistryImage_TimeoutsRef {
        RegistryImage_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for RegistryImage {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RegistryImage { }

impl terrars::ToListMappable for RegistryImage {
    type O = terrars::ListRef<RegistryImageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RegistryImage_ {
    fn extract_resource_type(&self) -> std::string::String {
        "docker_registry_image".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRegistryImage {
    pub tf_id: std::string::String,
    #[doc = "The name of the Docker image."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRegistryImage {
    pub fn build(self, stack: &mut terrars::Stack) -> RegistryImage {
        let out = RegistryImage(std::rc::Rc::new(RegistryImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RegistryImageData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                insecure_skip_verify: core::default::Default::default(),
                keep_remotely: core::default::Default::default(),
                name: self.name,
                triggers: core::default::Default::default(),
                auth_config: core::default::Default::default(),
                build: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RegistryImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RegistryImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RegistryImageRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `keep_remotely` after provisioning.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`"]
    pub fn keep_remotely(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_remotely", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sha256_digest` after provisioning.\nThe sha256 digest of the image."]
    pub fn sha256_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_digest", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `triggers` after provisioning.\nA map of arbitrary strings that, when changed, will force the `docker_registry_image` resource to be replaced. This can be used to repush a local image"]
    pub fn triggers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<RegistryImage_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RegistryImage_TimeoutsRef {
        RegistryImage_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct RegistryImageDynamic {
    auth_config: std::option::Option<terrars::DynamicBlock<RegistryImage_AuthConfig>>,
    build: std::option::Option<terrars::DynamicBlock<RegistryImage_Build>>,
}

#[derive(Serialize)]
pub struct RegistryImage_AuthConfig {
    address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RegistryImage_AuthConfig {
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

impl terrars::ToListMappable for RegistryImage_AuthConfig {
    type O = terrars::BlockAssignable<RegistryImage_AuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegistryImage_AuthConfig {
    #[doc = "The address of the Docker registry."]
    pub address: terrars::PrimField<std::string::String>,
}

impl BuildRegistryImage_AuthConfig {
    pub fn build(self) -> RegistryImage_AuthConfig {
        RegistryImage_AuthConfig {
            address: self.address,
            password: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RegistryImage_AuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RegistryImage_AuthConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RegistryImage_AuthConfigRef {
        RegistryImage_AuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegistryImage_AuthConfigRef {
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

#[derive(Serialize)]
pub struct RegistryImage_Build_AuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    host_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RegistryImage_Build_AuthConfig {
    #[doc = "Set the field `auth`.\nthe auth token"]
    pub fn set_auth(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\nthe user emal"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_token`.\nthe identity token"]
    pub fn set_identity_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nthe registry password"]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_token`.\nthe registry token"]
    pub fn set_registry_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_address`.\nthe server address"]
    pub fn set_server_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_name`.\nthe registry user name"]
    pub fn set_user_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RegistryImage_Build_AuthConfig {
    type O = terrars::BlockAssignable<RegistryImage_Build_AuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegistryImage_Build_AuthConfig {
    #[doc = "hostname of the registry"]
    pub host_name: terrars::PrimField<std::string::String>,
}

impl BuildRegistryImage_Build_AuthConfig {
    pub fn build(self) -> RegistryImage_Build_AuthConfig {
        RegistryImage_Build_AuthConfig {
            auth: core::default::Default::default(),
            email: core::default::Default::default(),
            host_name: self.host_name,
            identity_token: core::default::Default::default(),
            password: core::default::Default::default(),
            registry_token: core::default::Default::default(),
            server_address: core::default::Default::default(),
            user_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RegistryImage_Build_AuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RegistryImage_Build_AuthConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RegistryImage_Build_AuthConfigRef {
        RegistryImage_Build_AuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegistryImage_Build_AuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth` after provisioning.\nthe auth token"]
    pub fn auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nthe user emal"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `host_name` after provisioning.\nhostname of the registry"]
    pub fn host_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_token` after provisioning.\nthe identity token"]
    pub fn identity_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_token", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nthe registry password"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_token` after provisioning.\nthe registry token"]
    pub fn registry_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_token", self.base))
    }

    #[doc = "Get a reference to the value of field `server_address` after provisioning.\nthe server address"]
    pub fn server_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_address", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name` after provisioning.\nthe registry user name"]
    pub fn user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.base))
    }
}

#[derive(Serialize)]
pub struct RegistryImage_Build_Secrets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RegistryImage_Build_Secrets {
    #[doc = "Set the field `env`.\nEnvironment variable source of the secret"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src`.\nFile source of the secret. Takes precedence over `env`"]
    pub fn set_src(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.src = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RegistryImage_Build_Secrets {
    type O = terrars::BlockAssignable<RegistryImage_Build_Secrets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegistryImage_Build_Secrets {
    #[doc = "ID of the secret. By default, secrets are mounted to /run/secrets/<id>"]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildRegistryImage_Build_Secrets {
    pub fn build(self) -> RegistryImage_Build_Secrets {
        RegistryImage_Build_Secrets {
            env: core::default::Default::default(),
            id: self.id,
            src: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RegistryImage_Build_SecretsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RegistryImage_Build_SecretsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RegistryImage_Build_SecretsRef {
        RegistryImage_Build_SecretsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegistryImage_Build_SecretsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nEnvironment variable source of the secret"]
    pub fn env(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nID of the secret. By default, secrets are mounted to /run/secrets/<id>"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src` after provisioning.\nFile source of the secret. Takes precedence over `env`"]
    pub fn src(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.src", self.base))
    }
}

#[derive(Serialize)]
pub struct RegistryImage_Build_Ulimit {
    hard: terrars::PrimField<f64>,
    name: terrars::PrimField<std::string::String>,
    soft: terrars::PrimField<f64>,
}

impl RegistryImage_Build_Ulimit { }

impl terrars::ToListMappable for RegistryImage_Build_Ulimit {
    type O = terrars::BlockAssignable<RegistryImage_Build_Ulimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegistryImage_Build_Ulimit {
    #[doc = "soft limit"]
    pub hard: terrars::PrimField<f64>,
    #[doc = "type of ulimit, e.g. `nofile`"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "hard limit"]
    pub soft: terrars::PrimField<f64>,
}

impl BuildRegistryImage_Build_Ulimit {
    pub fn build(self) -> RegistryImage_Build_Ulimit {
        RegistryImage_Build_Ulimit {
            hard: self.hard,
            name: self.name,
            soft: self.soft,
        }
    }
}

#[allow(dead_code)]
pub struct RegistryImage_Build_UlimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RegistryImage_Build_UlimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RegistryImage_Build_UlimitRef {
        RegistryImage_Build_UlimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegistryImage_Build_UlimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hard` after provisioning.\nsoft limit"]
    pub fn hard(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hard", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\ntype of ulimit, e.g. `nofile`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `soft` after provisioning.\nhard limit"]
    pub fn soft(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.soft", self.base))
    }
}

#[derive(Serialize, Default)]
struct RegistryImage_BuildDynamic {
    auth_config: std::option::Option<terrars::DynamicBlock<RegistryImage_Build_AuthConfig>>,
    secrets: std::option::Option<terrars::DynamicBlock<RegistryImage_Build_Secrets>>,
    ulimit: std::option::Option<terrars::DynamicBlock<RegistryImage_Build_Ulimit>>,
}

#[derive(Serialize)]
pub struct RegistryImage_Build {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_contexts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_args: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_log_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    builder: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_from: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_to: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_parent: std::option::Option<terrars::PrimField<std::string::String>>,
    context: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_quota: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_set_cpus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_set_mems: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_shares: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_hosts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_remove: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    isolation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_swap: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provenance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_parent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sbom: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_opt: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shm_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    squash: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suppress_output: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_legacy_builder: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<std::vec::Vec<RegistryImage_Build_AuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets: std::option::Option<std::vec::Vec<RegistryImage_Build_Secrets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ulimit: std::option::Option<std::vec::Vec<RegistryImage_Build_Ulimit>>,
    dynamic: RegistryImage_BuildDynamic,
}

impl RegistryImage_Build {
    #[doc =
        "Set the field `additional_contexts`.\nA list of additional build contexts. Only supported when using a buildx builder. Example: `[\"name=path\", \"src = https://example.org\"}`. Please see https://docs.docker.com/reference/cli/docker/buildx/build/#build-context for more information."]
    pub fn set_additional_contexts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_contexts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `build_args`.\nPairs for build-time variables in the form of `ENDPOINT : \"https://example.com\"`"]
    pub fn set_build_args(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.build_args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `build_id`.\nBuildID is an optional identifier that can be passed together with the build request. The same identifier can be used to gracefully cancel the build with the cancel request."]
    pub fn set_build_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `build_log_file`.\nPath to a file where the buildx log are written to. Only available when `builder` is set. If not set, no logs are available. The path is taken as is, so make sure to use a path that is available."]
    pub fn set_build_log_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_log_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `builder`.\nThe name of the buildx builder to use. If BUILDX_BUILDER environment variable is set, it will be used. If left empty, the provider tries to resolve to the default builder - which might not always work. If you are in Windows, the legacy builder is used."]
    pub fn set_builder(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.builder = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_from`.\nExternal cache sources (e.g., `user/app:cache`, `type=local,src=path/to/dir`). Only supported when using a buildx builder."]
    pub fn set_cache_from(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cache_from = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_to`.\nCache export destinations (e.g., `user/app:cache`, `type=local,dest=path/to/dir`). Only supported when using a buildx builder."]
    pub fn set_cache_to(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cache_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cgroup_parent`.\nOptional parent cgroup for the container"]
    pub fn set_cgroup_parent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cgroup_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_period`.\nThe length of a CPU period in microseconds"]
    pub fn set_cpu_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_quota`.\nMicroseconds of CPU time that the container can get in a CPU period"]
    pub fn set_cpu_quota(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_quota = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_set_cpus`.\nCPUs in which to allow execution (e.g., `0-3`, `0`, `1`)"]
    pub fn set_cpu_set_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_set_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_set_mems`.\nMEMs in which to allow execution (`0-3`, `0`, `1`)"]
    pub fn set_cpu_set_mems(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_set_mems = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_shares`.\nCPU shares (relative weight)"]
    pub fn set_cpu_shares(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_shares = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile`.\nName of the Dockerfile. Defaults to `Dockerfile`."]
    pub fn set_dockerfile(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `extra_hosts`.\nA list of hostnames/IP mappings to add to the container’s /etc/hosts file. Specified in the form [\"hostname:IP\"]"]
    pub fn set_extra_hosts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.extra_hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_remove`.\nAlways remove intermediate containers"]
    pub fn set_force_remove(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.force_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `isolation`.\nIsolation represents the isolation technology of a container. The supported values are "]
    pub fn set_isolation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\nSet metadata for an image"]
    pub fn set_label(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nUser-defined key/value metadata"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nSet memory limit for build"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_swap`.\nTotal memory (memory + swap), -1 to enable unlimited swap"]
    pub fn set_memory_swap(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_swap = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_mode`.\nSet the networking mode for the RUN instructions during build"]
    pub fn set_network_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_cache`.\nDo not use the cache when building the image"]
    pub fn set_no_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `platform`.\nSet the target platform for the build. Defaults to `GOOS/GOARCH`. For more information see the [docker documentation](https://github.com/docker/buildx/blob/master/docs/reference/buildx.md#-set-the-target-platforms-for-the-build---platform)"]
    pub fn set_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provenance`.\nSet provenance attestation for the build. BuildKit v0.11+ adds provenance attestations by default, which creates OCI image manifests that some registries (like AWS Lambda) don't support. Set to `false` to disable. Valid values: `false`, `true`, `min`, `max`, `mode=min`, `mode=max`, or a full provenance specification. Only available when using a buildx builder."]
    pub fn set_provenance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provenance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pull_parent`.\nAttempt to pull the image even if an older image exists locally"]
    pub fn set_pull_parent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pull_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remote_context`.\nA Git repository URI or HTTP/HTTPS context URI. Will be ignored if `builder` is set."]
    pub fn set_remote_context(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.remote_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remove`.\nRemove intermediate containers after a successful build. Defaults to `true`."]
    pub fn set_remove(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sbom`.\nSet SBOM (Software Bill of Materials) attestation for the build. Set to `false` to disable. Valid values: `false`, `true`, or a full SBOM specification. Only available when using a buildx builder."]
    pub fn set_sbom(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sbom = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_opt`.\nThe security options"]
    pub fn set_security_opt(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_opt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_id`.\nSet an ID for the build session"]
    pub fn set_session_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shm_size`.\nSize of /dev/shm in bytes. The size must be greater than 0"]
    pub fn set_shm_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shm_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `squash`.\nIf true the new layers are squashed into a new image with a single new layer"]
    pub fn set_squash(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.squash = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suppress_output`.\nSuppress the build output and print image ID on success"]
    pub fn set_suppress_output(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.suppress_output = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nName and optionally a tag in the 'name:tag' format"]
    pub fn set_tag(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\nSet the target build stage to build"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_legacy_builder`.\nForce using the legacy Docker builder for image builds, even if buildx/buildkit would be available."]
    pub fn set_use_legacy_builder(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_legacy_builder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of the underlying builder to use"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RegistryImage_Build_AuthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secrets`.\n"]
    pub fn set_secrets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RegistryImage_Build_Secrets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secrets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ulimit`.\n"]
    pub fn set_ulimit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RegistryImage_Build_Ulimit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ulimit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ulimit = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RegistryImage_Build {
    type O = terrars::BlockAssignable<RegistryImage_Build>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegistryImage_Build {
    #[doc =
        "Value to specify the build context. Currently, only a `PATH` context is supported. You can use the helper function '${path.cwd}/context-dir'. This always refers to the local working directory, even when building images on remote hosts. Please see https://docs.docker.com/build/building/context/ for more information about build contexts."]
    pub context: terrars::PrimField<std::string::String>,
}

impl BuildRegistryImage_Build {
    pub fn build(self) -> RegistryImage_Build {
        RegistryImage_Build {
            additional_contexts: core::default::Default::default(),
            build_args: core::default::Default::default(),
            build_id: core::default::Default::default(),
            build_log_file: core::default::Default::default(),
            builder: core::default::Default::default(),
            cache_from: core::default::Default::default(),
            cache_to: core::default::Default::default(),
            cgroup_parent: core::default::Default::default(),
            context: self.context,
            cpu_period: core::default::Default::default(),
            cpu_quota: core::default::Default::default(),
            cpu_set_cpus: core::default::Default::default(),
            cpu_set_mems: core::default::Default::default(),
            cpu_shares: core::default::Default::default(),
            dockerfile: core::default::Default::default(),
            extra_hosts: core::default::Default::default(),
            force_remove: core::default::Default::default(),
            isolation: core::default::Default::default(),
            label: core::default::Default::default(),
            labels: core::default::Default::default(),
            memory: core::default::Default::default(),
            memory_swap: core::default::Default::default(),
            network_mode: core::default::Default::default(),
            no_cache: core::default::Default::default(),
            platform: core::default::Default::default(),
            provenance: core::default::Default::default(),
            pull_parent: core::default::Default::default(),
            remote_context: core::default::Default::default(),
            remove: core::default::Default::default(),
            sbom: core::default::Default::default(),
            security_opt: core::default::Default::default(),
            session_id: core::default::Default::default(),
            shm_size: core::default::Default::default(),
            squash: core::default::Default::default(),
            suppress_output: core::default::Default::default(),
            tag: core::default::Default::default(),
            target: core::default::Default::default(),
            use_legacy_builder: core::default::Default::default(),
            version: core::default::Default::default(),
            auth_config: core::default::Default::default(),
            secrets: core::default::Default::default(),
            ulimit: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RegistryImage_BuildRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RegistryImage_BuildRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RegistryImage_BuildRef {
        RegistryImage_BuildRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegistryImage_BuildRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_contexts` after provisioning.\nA list of additional build contexts. Only supported when using a buildx builder. Example: `[\"name=path\", \"src = https://example.org\"}`. Please see https://docs.docker.com/reference/cli/docker/buildx/build/#build-context for more information."]
    pub fn additional_contexts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_contexts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_args` after provisioning.\nPairs for build-time variables in the form of `ENDPOINT : \"https://example.com\"`"]
    pub fn build_args(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.build_args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_id` after provisioning.\nBuildID is an optional identifier that can be passed together with the build request. The same identifier can be used to gracefully cancel the build with the cancel request."]
    pub fn build_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `build_log_file` after provisioning.\nPath to a file where the buildx log are written to. Only available when `builder` is set. If not set, no logs are available. The path is taken as is, so make sure to use a path that is available."]
    pub fn build_log_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_log_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `builder` after provisioning.\nThe name of the buildx builder to use. If BUILDX_BUILDER environment variable is set, it will be used. If left empty, the provider tries to resolve to the default builder - which might not always work. If you are in Windows, the legacy builder is used."]
    pub fn builder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.builder", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_from` after provisioning.\nExternal cache sources (e.g., `user/app:cache`, `type=local,src=path/to/dir`). Only supported when using a buildx builder."]
    pub fn cache_from(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_from", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_to` after provisioning.\nCache export destinations (e.g., `user/app:cache`, `type=local,dest=path/to/dir`). Only supported when using a buildx builder."]
    pub fn cache_to(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_to", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cgroup_parent` after provisioning.\nOptional parent cgroup for the container"]
    pub fn cgroup_parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroup_parent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `context` after provisioning.\nValue to specify the build context. Currently, only a `PATH` context is supported. You can use the helper function '${path.cwd}/context-dir'. This always refers to the local working directory, even when building images on remote hosts. Please see https://docs.docker.com/build/building/context/ for more information about build contexts."]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_period` after provisioning.\nThe length of a CPU period in microseconds"]
    pub fn cpu_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_quota` after provisioning.\nMicroseconds of CPU time that the container can get in a CPU period"]
    pub fn cpu_quota(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_quota", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_set_cpus` after provisioning.\nCPUs in which to allow execution (e.g., `0-3`, `0`, `1`)"]
    pub fn cpu_set_cpus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_set_cpus", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_set_mems` after provisioning.\nMEMs in which to allow execution (`0-3`, `0`, `1`)"]
    pub fn cpu_set_mems(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_set_mems", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_shares` after provisioning.\nCPU shares (relative weight)"]
    pub fn cpu_shares(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_shares", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dockerfile` after provisioning.\nName of the Dockerfile. Defaults to `Dockerfile`."]
    pub fn dockerfile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile", self.base))
    }

    #[doc =
        "Get a reference to the value of field `extra_hosts` after provisioning.\nA list of hostnames/IP mappings to add to the container’s /etc/hosts file. Specified in the form [\"hostname:IP\"]"]
    pub fn extra_hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extra_hosts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `force_remove` after provisioning.\nAlways remove intermediate containers"]
    pub fn force_remove(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `isolation` after provisioning.\nIsolation represents the isolation technology of a container. The supported values are "]
    pub fn isolation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\nSet metadata for an image"]
    pub fn label(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined key/value metadata"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\nSet memory limit for build"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_swap` after provisioning.\nTotal memory (memory + swap), -1 to enable unlimited swap"]
    pub fn memory_swap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_mode` after provisioning.\nSet the networking mode for the RUN instructions during build"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_cache` after provisioning.\nDo not use the cache when building the image"]
    pub fn no_cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `platform` after provisioning.\nSet the target platform for the build. Defaults to `GOOS/GOARCH`. For more information see the [docker documentation](https://github.com/docker/buildx/blob/master/docs/reference/buildx.md#-set-the-target-platforms-for-the-build---platform)"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provenance` after provisioning.\nSet provenance attestation for the build. BuildKit v0.11+ adds provenance attestations by default, which creates OCI image manifests that some registries (like AWS Lambda) don't support. Set to `false` to disable. Valid values: `false`, `true`, `min`, `max`, `mode=min`, `mode=max`, or a full provenance specification. Only available when using a buildx builder."]
    pub fn provenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provenance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pull_parent` after provisioning.\nAttempt to pull the image even if an older image exists locally"]
    pub fn pull_parent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pull_parent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remote_context` after provisioning.\nA Git repository URI or HTTP/HTTPS context URI. Will be ignored if `builder` is set."]
    pub fn remote_context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_context", self.base))
    }

    #[doc =
        "Get a reference to the value of field `remove` after provisioning.\nRemove intermediate containers after a successful build. Defaults to `true`."]
    pub fn remove(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sbom` after provisioning.\nSet SBOM (Software Bill of Materials) attestation for the build. Set to `false` to disable. Valid values: `false`, `true`, or a full SBOM specification. Only available when using a buildx builder."]
    pub fn sbom(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sbom", self.base))
    }

    #[doc = "Get a reference to the value of field `security_opt` after provisioning.\nThe security options"]
    pub fn security_opt(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_opt", self.base))
    }

    #[doc = "Get a reference to the value of field `session_id` after provisioning.\nSet an ID for the build session"]
    pub fn session_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `shm_size` after provisioning.\nSize of /dev/shm in bytes. The size must be greater than 0"]
    pub fn shm_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shm_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `squash` after provisioning.\nIf true the new layers are squashed into a new image with a single new layer"]
    pub fn squash(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suppress_output` after provisioning.\nSuppress the build output and print image ID on success"]
    pub fn suppress_output(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suppress_output", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nName and optionally a tag in the 'name:tag' format"]
    pub fn tag(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\nSet the target build stage to build"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_legacy_builder` after provisioning.\nForce using the legacy Docker builder for image builds, even if buildx/buildkit would be available."]
    pub fn use_legacy_builder(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_legacy_builder", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the underlying builder to use"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<RegistryImage_Build_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> terrars::ListRef<RegistryImage_Build_SecretsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets", self.base))
    }

    #[doc = "Get a reference to the value of field `ulimit` after provisioning.\n"]
    pub fn ulimit(&self) -> terrars::ListRef<RegistryImage_Build_UlimitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ulimit", self.base))
    }
}

#[derive(Serialize)]
pub struct RegistryImage_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RegistryImage_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RegistryImage_Timeouts {
    type O = terrars::BlockAssignable<RegistryImage_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRegistryImage_Timeouts {}

impl BuildRegistryImage_Timeouts {
    pub fn build(self) -> RegistryImage_Timeouts {
        RegistryImage_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RegistryImage_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RegistryImage_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RegistryImage_TimeoutsRef {
        RegistryImage_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RegistryImage_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
