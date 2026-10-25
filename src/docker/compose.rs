#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct ComposeData_ {
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
    config_paths: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env_files: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove_orphans: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct Compose_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComposeData_>,
}

#[derive(Clone)]
pub struct Compose(std::rc::Rc<Compose_>);

impl Compose {
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

    #[doc =
        "Set the field `env_files`.\nOptional list of env files to load before parsing the Compose configuration. If omitted, Compose uses the default `.env` behavior."]
    pub fn set_env_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().env_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profiles`.\nOptional list of Compose profiles to enable."]
    pub fn set_profiles(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().profiles = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_directory`.\nOptional project directory used as the Compose working directory. If omitted, Compose uses the directory of the first file in `config_paths`."]
    pub fn set_project_directory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_name`.\nOptional Compose project name. If omitted, Compose derives the project name the same way as the CLI."]
    pub fn set_project_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remove_orphans`.\nIf `true`, remove containers for services that are no longer present in the Compose configuration during apply and destroy."]
    pub fn set_remove_orphans(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_orphans = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait`.\nIf `true`, wait until services reach the running or healthy state before returning from apply."]
    pub fn set_wait(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_timeout`.\nOptional duration for `wait`, for example `30s` or `2m`."]
    pub fn set_wait_timeout(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().wait_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `config_paths` after provisioning.\nOne or more Compose file paths, equivalent to repeating the `-f` flag with `docker compose`."]
    pub fn config_paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_paths", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_files` after provisioning.\nOptional list of env files to load before parsing the Compose configuration. If omitted, Compose uses the default `.env` behavior."]
    pub fn env_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env_files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe Compose project name used as the Terraform resource ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `profiles` after provisioning.\nOptional list of Compose profiles to enable."]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_directory` after provisioning.\nOptional project directory used as the Compose working directory. If omitted, Compose uses the directory of the first file in `config_paths`."]
    pub fn project_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_directory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_name` after provisioning.\nOptional Compose project name. If omitted, Compose derives the project name the same way as the CLI."]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_orphans` after provisioning.\nIf `true`, remove containers for services that are no longer present in the Compose configuration during apply and destroy."]
    pub fn remove_orphans(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove_orphans", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nIf `true`, wait until services reach the running or healthy state before returning from apply."]
    pub fn wait(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_timeout` after provisioning.\nOptional duration for `wait`, for example `30s` or `2m`."]
    pub fn wait_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_timeout", self.extract_ref()))
    }
}

impl terrars::Referable for Compose {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Compose { }

impl terrars::ToListMappable for Compose {
    type O = terrars::ListRef<ComposeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Compose_ {
    fn extract_resource_type(&self) -> std::string::String {
        "docker_compose".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCompose {
    pub tf_id: std::string::String,
    #[doc = "One or more Compose file paths, equivalent to repeating the `-f` flag with `docker compose`."]
    pub config_paths: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCompose {
    pub fn build(self, stack: &mut terrars::Stack) -> Compose {
        let out = Compose(std::rc::Rc::new(Compose_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComposeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                config_paths: self.config_paths,
                env_files: core::default::Default::default(),
                profiles: core::default::Default::default(),
                project_directory: core::default::Default::default(),
                project_name: core::default::Default::default(),
                remove_orphans: core::default::Default::default(),
                wait: core::default::Default::default(),
                wait_timeout: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComposeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComposeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComposeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config_paths` after provisioning.\nOne or more Compose file paths, equivalent to repeating the `-f` flag with `docker compose`."]
    pub fn config_paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_paths", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_files` after provisioning.\nOptional list of env files to load before parsing the Compose configuration. If omitted, Compose uses the default `.env` behavior."]
    pub fn env_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env_files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe Compose project name used as the Terraform resource ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `profiles` after provisioning.\nOptional list of Compose profiles to enable."]
    pub fn profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_directory` after provisioning.\nOptional project directory used as the Compose working directory. If omitted, Compose uses the directory of the first file in `config_paths`."]
    pub fn project_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_directory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_name` after provisioning.\nOptional Compose project name. If omitted, Compose derives the project name the same way as the CLI."]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_orphans` after provisioning.\nIf `true`, remove containers for services that are no longer present in the Compose configuration during apply and destroy."]
    pub fn remove_orphans(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove_orphans", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nIf `true`, wait until services reach the running or healthy state before returning from apply."]
    pub fn wait(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_timeout` after provisioning.\nOptional duration for `wait`, for example `30s` or `2m`."]
    pub fn wait_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_timeout", self.extract_ref()))
    }
}
