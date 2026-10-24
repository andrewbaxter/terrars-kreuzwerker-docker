use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct PluginData {
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
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_disable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant_all_permissions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant_permissions: std::option::Option<std::vec::Vec<PluginGrantPermissionsEl>>,
    dynamic: PluginDynamic,
}

struct Plugin_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PluginData>,
}

#[derive(Clone)]
pub struct Plugin(std::rc::Rc<Plugin_>);

impl Plugin {
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

    #[doc = "Set the field `alias`.\nDocker Plugin alias"]
    pub fn set_alias(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_timeout`.\nHTTP client timeout to enable the plugin"]
    pub fn set_enable_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().enable_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nIf `true` the plugin is enabled. Defaults to `true`"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\nThe environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn set_env(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\nIf true, then the plugin is destroyed forcibly"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_disable`.\nIf true, then the plugin is disabled forcibly"]
    pub fn set_force_disable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_disable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grant_all_permissions`.\nIf true, grant all permissions necessary to run the plugin"]
    pub fn set_grant_all_permissions(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().grant_all_permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grant_permissions`.\n"]
    pub fn set_grant_permissions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PluginGrantPermissionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grant_permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grant_permissions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\nDocker Plugin alias"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_timeout` after provisioning.\nHTTP client timeout to enable the plugin"]
    pub fn enable_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIf `true` the plugin is enabled. Defaults to `true`"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nThe environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nIf true, then the plugin is destroyed forcibly"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_disable` after provisioning.\nIf true, then the plugin is disabled forcibly"]
    pub fn force_disable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_disable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grant_all_permissions` after provisioning.\nIf true, grant all permissions necessary to run the plugin"]
    pub fn grant_all_permissions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_all_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDocker Plugin name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugin_reference` after provisioning.\nDocker Plugin Reference"]
    pub fn plugin_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_reference", self.extract_ref()))
    }
}

impl terrars::Referable for Plugin {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Plugin { }

impl terrars::ToListMappable for Plugin {
    type O = terrars::ListRef<PluginRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Plugin_ {
    fn extract_resource_type(&self) -> std::string::String {
        "docker_plugin".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPlugin {
    pub tf_id: std::string::String,
    #[doc = "Docker Plugin name"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildPlugin {
    pub fn build(self, stack: &mut terrars::Stack) -> Plugin {
        let out = Plugin(std::rc::Rc::new(Plugin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PluginData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias: core::default::Default::default(),
                enable_timeout: core::default::Default::default(),
                enabled: core::default::Default::default(),
                env: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                force_disable: core::default::Default::default(),
                grant_all_permissions: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                grant_permissions: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PluginRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\nDocker Plugin alias"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_timeout` after provisioning.\nHTTP client timeout to enable the plugin"]
    pub fn enable_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIf `true` the plugin is enabled. Defaults to `true`"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nThe environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nIf true, then the plugin is destroyed forcibly"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_disable` after provisioning.\nIf true, then the plugin is disabled forcibly"]
    pub fn force_disable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_disable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grant_all_permissions` after provisioning.\nIf true, grant all permissions necessary to run the plugin"]
    pub fn grant_all_permissions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_all_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDocker Plugin name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugin_reference` after provisioning.\nDocker Plugin Reference"]
    pub fn plugin_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_reference", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PluginGrantPermissionsEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl PluginGrantPermissionsEl { }

impl terrars::ToListMappable for PluginGrantPermissionsEl {
    type O = terrars::BlockAssignable<PluginGrantPermissionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPluginGrantPermissionsEl {
    #[doc = "The name of the permission"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the permission"]
    pub value: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPluginGrantPermissionsEl {
    pub fn build(self) -> PluginGrantPermissionsEl {
        PluginGrantPermissionsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct PluginGrantPermissionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PluginGrantPermissionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PluginGrantPermissionsElRef {
        PluginGrantPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PluginGrantPermissionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the permission"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the permission"]
    pub fn value(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct PluginDynamic {
    grant_permissions: std::option::Option<terrars::DynamicBlock<PluginGrantPermissionsEl>>,
}
