#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct BuildxBuilderData_ {
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
    append: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bootstrap: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buildkit_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buildkit_flags: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "use", skip_serializing_if = "std::option::Option::is_none")]
    use_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_container: std::option::Option<std::vec::Vec<BuildxBuilder_DockerContainer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes: std::option::Option<std::vec::Vec<BuildxBuilder_Kubernetes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std::option::Option<std::vec::Vec<BuildxBuilder_Remote>>,
    dynamic: BuildxBuilderDynamic,
}

struct BuildxBuilder_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BuildxBuilderData_>,
}

#[derive(Clone)]
pub struct BuildxBuilder(std::rc::Rc<BuildxBuilder_>);

impl BuildxBuilder {
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

    #[doc = "Set the field `append`.\nAppend a node to builder instead of changing it"]
    pub fn set_append(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().append = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bootstrap`.\nAutomatically boot the builder after creation. Defaults to `false`"]
    pub fn set_bootstrap(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bootstrap = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buildkit_config`.\nBuildKit daemon config file"]
    pub fn set_buildkit_config(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().buildkit_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buildkit_flags`.\nBuildKit flags to set for the builder."]
    pub fn set_buildkit_flags(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().buildkit_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver`.\nThe driver to use for the Buildx builder (e.g., docker-container, kubernetes)."]
    pub fn set_driver(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().driver = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `driver_options`.\nAdditional options for the Buildx driver in the form of `key=value,...`. These options are driver-specific."]
    pub fn set_driver_options(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().driver_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endpoint`.\nThe endpoint or context to use for the Buildx builder, where context is the name of a context from docker context ls and endpoint is the address for Docker socket (eg. DOCKER_HOST value). By default, the current Docker configuration is used for determining the context/endpoint value."]
    pub fn set_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the Buildx builder. IF not specified, a random name will be generated."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node`.\nCreate/modify node with given name"]
    pub fn set_node(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform`.\nFixed platforms for current node"]
    pub fn set_platform(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_`.\nSet the current builder instance as the default for the current context."]
    pub fn set_use(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `docker_container`.\n"]
    pub fn set_docker_container(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BuildxBuilder_DockerContainer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().docker_container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.docker_container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubernetes`.\n"]
    pub fn set_kubernetes(self, v: impl std::convert::Into<terrars::BlockAssignable<BuildxBuilder_Kubernetes>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kubernetes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kubernetes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(self, v: impl std::convert::Into<terrars::BlockAssignable<BuildxBuilder_Remote>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `append` after provisioning.\nAppend a node to builder instead of changing it"]
    pub fn append(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bootstrap` after provisioning.\nAutomatically boot the builder after creation. Defaults to `false`"]
    pub fn bootstrap(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `buildkit_config` after provisioning.\nBuildKit daemon config file"]
    pub fn buildkit_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buildkit_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `buildkit_flags` after provisioning.\nBuildKit flags to set for the builder."]
    pub fn buildkit_flags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buildkit_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver` after provisioning.\nThe driver to use for the Buildx builder (e.g., docker-container, kubernetes)."]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver_options` after provisioning.\nAdditional options for the Buildx driver in the form of `key=value,...`. These options are driver-specific."]
    pub fn driver_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint or context to use for the Buildx builder, where context is the name of a context from docker context ls and endpoint is the address for Docker socket (eg. DOCKER_HOST value). By default, the current Docker configuration is used for determining the context/endpoint value."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Buildx builder. IF not specified, a random name will be generated."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node` after provisioning.\nCreate/modify node with given name"]
    pub fn node(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\nFixed platforms for current node"]
    pub fn platform(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_` after provisioning.\nSet the current builder instance as the default for the current context."]
    pub fn use_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `docker_container` after provisioning.\n"]
    pub fn docker_container(&self) -> terrars::ListRef<BuildxBuilder_DockerContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]
    pub fn kubernetes(&self) -> terrars::ListRef<BuildxBuilder_KubernetesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(&self) -> terrars::ListRef<BuildxBuilder_RemoteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.extract_ref()))
    }
}

impl terrars::Referable for BuildxBuilder {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BuildxBuilder { }

impl terrars::ToListMappable for BuildxBuilder {
    type O = terrars::ListRef<BuildxBuilderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BuildxBuilder_ {
    fn extract_resource_type(&self) -> std::string::String {
        "docker_buildx_builder".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBuildxBuilder {
    pub tf_id: std::string::String,
}

impl BuildBuildxBuilder {
    pub fn build(self, stack: &mut terrars::Stack) -> BuildxBuilder {
        let out = BuildxBuilder(std::rc::Rc::new(BuildxBuilder_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BuildxBuilderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                append: core::default::Default::default(),
                bootstrap: core::default::Default::default(),
                buildkit_config: core::default::Default::default(),
                buildkit_flags: core::default::Default::default(),
                driver: core::default::Default::default(),
                driver_options: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                node: core::default::Default::default(),
                platform: core::default::Default::default(),
                use_: core::default::Default::default(),
                docker_container: core::default::Default::default(),
                kubernetes: core::default::Default::default(),
                remote: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BuildxBuilderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BuildxBuilderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BuildxBuilderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `append` after provisioning.\nAppend a node to builder instead of changing it"]
    pub fn append(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bootstrap` after provisioning.\nAutomatically boot the builder after creation. Defaults to `false`"]
    pub fn bootstrap(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `buildkit_config` after provisioning.\nBuildKit daemon config file"]
    pub fn buildkit_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buildkit_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `buildkit_flags` after provisioning.\nBuildKit flags to set for the builder."]
    pub fn buildkit_flags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buildkit_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver` after provisioning.\nThe driver to use for the Buildx builder (e.g., docker-container, kubernetes)."]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver_options` after provisioning.\nAdditional options for the Buildx driver in the form of `key=value,...`. These options are driver-specific."]
    pub fn driver_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint or context to use for the Buildx builder, where context is the name of a context from docker context ls and endpoint is the address for Docker socket (eg. DOCKER_HOST value). By default, the current Docker configuration is used for determining the context/endpoint value."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Buildx builder. IF not specified, a random name will be generated."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node` after provisioning.\nCreate/modify node with given name"]
    pub fn node(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\nFixed platforms for current node"]
    pub fn platform(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_` after provisioning.\nSet the current builder instance as the default for the current context."]
    pub fn use_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `docker_container` after provisioning.\n"]
    pub fn docker_container(&self) -> terrars::ListRef<BuildxBuilder_DockerContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]
    pub fn kubernetes(&self) -> terrars::ListRef<BuildxBuilder_KubernetesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(&self) -> terrars::ListRef<BuildxBuilder_RemoteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BuildxBuilderDynamic {
    docker_container: std::option::Option<terrars::DynamicBlock<BuildxBuilder_DockerContainer>>,
    kubernetes: std::option::Option<terrars::DynamicBlock<BuildxBuilder_Kubernetes>>,
    remote: std::option::Option<terrars::DynamicBlock<BuildxBuilder_Remote>>,
}

#[derive(Serialize)]
pub struct BuildxBuilder_DockerContainer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_quota: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_shares: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpuset_cpus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpuset_mems: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_load: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_swap: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BuildxBuilder_DockerContainer {
    #[doc =
        "Set the field `cgroup_parent`.\nSets the cgroup parent of the container if Docker is using the \"cgroupfs\" driver."]
    pub fn set_cgroup_parent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cgroup_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_period`.\nSets the CPU CFS scheduler period for the container."]
    pub fn set_cpu_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_quota`.\nImposes a CPU CFS quota on the container."]
    pub fn set_cpu_quota(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_quota = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_shares`.\nConfigures CPU shares (relative weight) of the container."]
    pub fn set_cpu_shares(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_shares = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpuset_cpus`.\nLimits the set of CPU cores the container can use."]
    pub fn set_cpuset_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpuset_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpuset_mems`.\nLimits the set of CPU memory nodes the container can use."]
    pub fn set_cpuset_mems(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpuset_mems = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_load`.\nAutomatically load images to the Docker Engine image store. Defaults to `false`"]
    pub fn set_default_load(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_load = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\nSets environment variables in the container."]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\nSets the BuildKit image to use for the container."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nSets the amount of memory the container can use."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_swap`.\nSets the memory swap limit for the container."]
    pub fn set_memory_swap(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_swap = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nSets the network mode for the container."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restart_policy`.\nSets the container's restart policy."]
    pub fn set_restart_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.restart_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BuildxBuilder_DockerContainer {
    type O = terrars::BlockAssignable<BuildxBuilder_DockerContainer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBuildxBuilder_DockerContainer {}

impl BuildBuildxBuilder_DockerContainer {
    pub fn build(self) -> BuildxBuilder_DockerContainer {
        BuildxBuilder_DockerContainer {
            cgroup_parent: core::default::Default::default(),
            cpu_period: core::default::Default::default(),
            cpu_quota: core::default::Default::default(),
            cpu_shares: core::default::Default::default(),
            cpuset_cpus: core::default::Default::default(),
            cpuset_mems: core::default::Default::default(),
            default_load: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            memory: core::default::Default::default(),
            memory_swap: core::default::Default::default(),
            network: core::default::Default::default(),
            restart_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BuildxBuilder_DockerContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BuildxBuilder_DockerContainerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BuildxBuilder_DockerContainerRef {
        BuildxBuilder_DockerContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BuildxBuilder_DockerContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cgroup_parent` after provisioning.\nSets the cgroup parent of the container if Docker is using the \"cgroupfs\" driver."]
    pub fn cgroup_parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroup_parent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_period` after provisioning.\nSets the CPU CFS scheduler period for the container."]
    pub fn cpu_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_quota` after provisioning.\nImposes a CPU CFS quota on the container."]
    pub fn cpu_quota(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_quota", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_shares` after provisioning.\nConfigures CPU shares (relative weight) of the container."]
    pub fn cpu_shares(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_shares", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpuset_cpus` after provisioning.\nLimits the set of CPU cores the container can use."]
    pub fn cpuset_cpus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpuset_cpus", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpuset_mems` after provisioning.\nLimits the set of CPU memory nodes the container can use."]
    pub fn cpuset_mems(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpuset_mems", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_load` after provisioning.\nAutomatically load images to the Docker Engine image store. Defaults to `false`"]
    pub fn default_load(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_load", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nSets environment variables in the container."]
    pub fn env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nSets the BuildKit image to use for the container."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nSets the amount of memory the container can use."]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_swap` after provisioning.\nSets the memory swap limit for the container."]
    pub fn memory_swap(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nSets the network mode for the container."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restart_policy` after provisioning.\nSets the container's restart policy."]
    pub fn restart_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct BuildxBuilder_Kubernetes_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BuildxBuilder_Kubernetes_Limits {
    #[doc = "Set the field `cpu`.\nCPU limit for the Kubernetes pod."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\nEphemeral storage limit for the Kubernetes pod."]
    pub fn set_ephemeral_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ephemeral_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nMemory limit for the Kubernetes pod."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BuildxBuilder_Kubernetes_Limits {
    type O = terrars::BlockAssignable<BuildxBuilder_Kubernetes_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBuildxBuilder_Kubernetes_Limits {}

impl BuildBuildxBuilder_Kubernetes_Limits {
    pub fn build(self) -> BuildxBuilder_Kubernetes_Limits {
        BuildxBuilder_Kubernetes_Limits {
            cpu: core::default::Default::default(),
            ephemeral_storage: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BuildxBuilder_Kubernetes_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BuildxBuilder_Kubernetes_LimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BuildxBuilder_Kubernetes_LimitsRef {
        BuildxBuilder_Kubernetes_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BuildxBuilder_Kubernetes_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU limit for the Kubernetes pod."]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ephemeral_storage` after provisioning.\nEphemeral storage limit for the Kubernetes pod."]
    pub fn ephemeral_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ephemeral_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\nMemory limit for the Kubernetes pod."]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct BuildxBuilder_Kubernetes_Qemu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    install: std::option::Option<terrars::PrimField<bool>>,
}

impl BuildxBuilder_Kubernetes_Qemu {
    #[doc = "Set the field `image`.\nSets the QEMU emulation image."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `install`.\nInstall QEMU emulation for multi-platform support."]
    pub fn set_install(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.install = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BuildxBuilder_Kubernetes_Qemu {
    type O = terrars::BlockAssignable<BuildxBuilder_Kubernetes_Qemu>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBuildxBuilder_Kubernetes_Qemu {}

impl BuildBuildxBuilder_Kubernetes_Qemu {
    pub fn build(self) -> BuildxBuilder_Kubernetes_Qemu {
        BuildxBuilder_Kubernetes_Qemu {
            image: core::default::Default::default(),
            install: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BuildxBuilder_Kubernetes_QemuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BuildxBuilder_Kubernetes_QemuRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BuildxBuilder_Kubernetes_QemuRef {
        BuildxBuilder_Kubernetes_QemuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BuildxBuilder_Kubernetes_QemuRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\nSets the QEMU emulation image."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `install` after provisioning.\nInstall QEMU emulation for multi-platform support."]
    pub fn install(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.install", self.base))
    }
}

#[derive(Serialize)]
pub struct BuildxBuilder_Kubernetes_Requests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BuildxBuilder_Kubernetes_Requests {
    #[doc = "Set the field `cpu`.\nCPU limit for the Kubernetes pod."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\nEphemeral storage limit for the Kubernetes pod."]
    pub fn set_ephemeral_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ephemeral_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nMemory limit for the Kubernetes pod."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BuildxBuilder_Kubernetes_Requests {
    type O = terrars::BlockAssignable<BuildxBuilder_Kubernetes_Requests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBuildxBuilder_Kubernetes_Requests {}

impl BuildBuildxBuilder_Kubernetes_Requests {
    pub fn build(self) -> BuildxBuilder_Kubernetes_Requests {
        BuildxBuilder_Kubernetes_Requests {
            cpu: core::default::Default::default(),
            ephemeral_storage: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BuildxBuilder_Kubernetes_RequestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BuildxBuilder_Kubernetes_RequestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BuildxBuilder_Kubernetes_RequestsRef {
        BuildxBuilder_Kubernetes_RequestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BuildxBuilder_Kubernetes_RequestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nCPU limit for the Kubernetes pod."]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ephemeral_storage` after provisioning.\nEphemeral storage limit for the Kubernetes pod."]
    pub fn ephemeral_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ephemeral_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\nMemory limit for the Kubernetes pod."]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize, Default)]
struct BuildxBuilder_KubernetesDynamic {
    limits: std::option::Option<terrars::DynamicBlock<BuildxBuilder_Kubernetes_Limits>>,
    qemu: std::option::Option<terrars::DynamicBlock<BuildxBuilder_Kubernetes_Qemu>>,
    requests: std::option::Option<terrars::DynamicBlock<BuildxBuilder_Kubernetes_Requests>>,
}

#[derive(Serialize)]
pub struct BuildxBuilder_Kubernetes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_load: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    loadbalance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nodeselector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rootless: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedulername: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serviceaccount: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tolerations: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<BuildxBuilder_Kubernetes_Limits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qemu: std::option::Option<std::vec::Vec<BuildxBuilder_Kubernetes_Qemu>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std::option::Option<std::vec::Vec<BuildxBuilder_Kubernetes_Requests>>,
    dynamic: BuildxBuilder_KubernetesDynamic,
}

impl BuildxBuilder_Kubernetes {
    #[doc = "Set the field `annotations`.\nSets additional annotations on the deployments and pods."]
    pub fn set_annotations(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_load`.\nAutomatically load images to the Docker Engine image store. Defaults to `false`"]
    pub fn set_default_load(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_load = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\nSets the image to use for running BuildKit."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nSets additional labels on the deployments and pods."]
    pub fn set_labels(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `loadbalance`.\nLoad-balancing strategy (sticky or random)."]
    pub fn set_loadbalance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.loadbalance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\nSets the Kubernetes namespace."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodeselector`.\nSets the pod's nodeSelector label(s)."]
    pub fn set_nodeselector(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nodeselector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicas`.\nSets the number of Pod replicas to create."]
    pub fn set_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rootless`.\nRun the container as a non-root user."]
    pub fn set_rootless(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rootless = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedulername`.\nSets the scheduler responsible for scheduling the pod."]
    pub fn set_schedulername(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedulername = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serviceaccount`.\nSets the pod's serviceAccountName."]
    pub fn set_serviceaccount(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.serviceaccount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nSet the timeout limit for pod provisioning."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tolerations`.\nConfigures the pod's taint toleration."]
    pub fn set_tolerations(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tolerations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BuildxBuilder_Kubernetes_Limits>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `qemu`.\n"]
    pub fn set_qemu(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BuildxBuilder_Kubernetes_Qemu>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.qemu = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.qemu = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BuildxBuilder_Kubernetes_Requests>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.requests = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.requests = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BuildxBuilder_Kubernetes {
    type O = terrars::BlockAssignable<BuildxBuilder_Kubernetes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBuildxBuilder_Kubernetes {}

impl BuildBuildxBuilder_Kubernetes {
    pub fn build(self) -> BuildxBuilder_Kubernetes {
        BuildxBuilder_Kubernetes {
            annotations: core::default::Default::default(),
            default_load: core::default::Default::default(),
            image: core::default::Default::default(),
            labels: core::default::Default::default(),
            loadbalance: core::default::Default::default(),
            namespace: core::default::Default::default(),
            nodeselector: core::default::Default::default(),
            replicas: core::default::Default::default(),
            rootless: core::default::Default::default(),
            schedulername: core::default::Default::default(),
            serviceaccount: core::default::Default::default(),
            timeout: core::default::Default::default(),
            tolerations: core::default::Default::default(),
            limits: core::default::Default::default(),
            qemu: core::default::Default::default(),
            requests: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BuildxBuilder_KubernetesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BuildxBuilder_KubernetesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BuildxBuilder_KubernetesRef {
        BuildxBuilder_KubernetesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BuildxBuilder_KubernetesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nSets additional annotations on the deployments and pods."]
    pub fn annotations(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_load` after provisioning.\nAutomatically load images to the Docker Engine image store. Defaults to `false`"]
    pub fn default_load(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_load", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nSets the image to use for running BuildKit."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSets additional labels on the deployments and pods."]
    pub fn labels(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `loadbalance` after provisioning.\nLoad-balancing strategy (sticky or random)."]
    pub fn loadbalance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.loadbalance", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nSets the Kubernetes namespace."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nodeselector` after provisioning.\nSets the pod's nodeSelector label(s)."]
    pub fn nodeselector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nodeselector", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replicas` after provisioning.\nSets the number of Pod replicas to create."]
    pub fn replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rootless` after provisioning.\nRun the container as a non-root user."]
    pub fn rootless(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rootless", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedulername` after provisioning.\nSets the scheduler responsible for scheduling the pod."]
    pub fn schedulername(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedulername", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serviceaccount` after provisioning.\nSets the pod's serviceAccountName."]
    pub fn serviceaccount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serviceaccount", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nSet the timeout limit for pod provisioning."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tolerations` after provisioning.\nConfigures the pod's taint toleration."]
    pub fn tolerations(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tolerations", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<BuildxBuilder_Kubernetes_LimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `qemu` after provisioning.\n"]
    pub fn qemu(&self) -> terrars::ListRef<BuildxBuilder_Kubernetes_QemuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qemu", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(&self) -> terrars::ListRef<BuildxBuilder_Kubernetes_RequestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct BuildxBuilder_Remote {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cacert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_load: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    servername: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BuildxBuilder_Remote {
    #[doc = "Set the field `cacert`.\nAbsolute path to the TLS certificate authority used for validation."]
    pub fn set_cacert(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cacert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cert`.\nAbsolute path to the TLS client certificate to present to buildkitd."]
    pub fn set_cert(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cert = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_load`.\nAutomatically load images to the Docker Engine image store. Defaults to `false`"]
    pub fn set_default_load(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_load = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nSets the TLS client key."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `servername`.\nTLS server name used in requests."]
    pub fn set_servername(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.servername = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BuildxBuilder_Remote {
    type O = terrars::BlockAssignable<BuildxBuilder_Remote>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBuildxBuilder_Remote {}

impl BuildBuildxBuilder_Remote {
    pub fn build(self) -> BuildxBuilder_Remote {
        BuildxBuilder_Remote {
            cacert: core::default::Default::default(),
            cert: core::default::Default::default(),
            default_load: core::default::Default::default(),
            key: core::default::Default::default(),
            servername: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BuildxBuilder_RemoteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BuildxBuilder_RemoteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BuildxBuilder_RemoteRef {
        BuildxBuilder_RemoteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BuildxBuilder_RemoteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cacert` after provisioning.\nAbsolute path to the TLS certificate authority used for validation."]
    pub fn cacert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cacert", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cert` after provisioning.\nAbsolute path to the TLS client certificate to present to buildkitd."]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_load` after provisioning.\nAutomatically load images to the Docker Engine image store. Defaults to `false`"]
    pub fn default_load(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_load", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nSets the TLS client key."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `servername` after provisioning.\nTLS server name used in requests."]
    pub fn servername(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.servername", self.base))
    }
}
