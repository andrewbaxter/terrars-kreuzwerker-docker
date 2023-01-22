use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct PluginData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_disable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grant_all_permissions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grant_permissions: Option<Vec<PluginGrantPermissionsEl>>,
    dynamic: PluginDynamic,
}

struct Plugin_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PluginData>,
}

#[derive(Clone)]
pub struct Plugin(Rc<Plugin_>);

impl Plugin {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `alias`.\nDocker Plugin alias"]
    pub fn set_alias(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alias = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_timeout`.\nHTTP client timeout to enable the plugin"]
    pub fn set_enable_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().enable_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nIf `true` the plugin is enabled. Defaults to `true`"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\nThe environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn set_env(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().env = Some(v.into());
        self
    }

    #[doc= "Set the field `force_destroy`.\nIf true, then the plugin is destroyed forcibly"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `force_disable`.\nIf true, then the plugin is disabled forcibly"]
    pub fn set_force_disable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_disable = Some(v.into());
        self
    }

    #[doc= "Set the field `grant_all_permissions`.\nIf true, grant all permissions necessary to run the plugin"]
    pub fn set_grant_all_permissions(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().grant_all_permissions = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `grant_permissions`.\n"]
    pub fn set_grant_permissions(self, v: impl Into<BlockAssignable<PluginGrantPermissionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grant_permissions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grant_permissions = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `alias` after provisioning.\nDocker Plugin alias"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_timeout` after provisioning.\nHTTP client timeout to enable the plugin"]
    pub fn enable_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIf `true` the plugin is enabled. Defaults to `true`"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nThe environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_destroy` after provisioning.\nIf true, then the plugin is destroyed forcibly"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_disable` after provisioning.\nIf true, then the plugin is disabled forcibly"]
    pub fn force_disable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_disable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `grant_all_permissions` after provisioning.\nIf true, grant all permissions necessary to run the plugin"]
    pub fn grant_all_permissions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_all_permissions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nDocker Plugin name"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plugin_reference` after provisioning.\nDocker Plugin Reference"]
    pub fn plugin_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugin_reference", self.extract_ref()))
    }
}

impl Resource for Plugin {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for Plugin {
    type O = ListRef<PluginRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Plugin_ {
    fn extract_resource_type(&self) -> String {
        "docker_plugin".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPlugin {
    pub tf_id: String,
    #[doc= "Docker Plugin name"]
    pub name: PrimField<String>,
}

impl BuildPlugin {
    pub fn build(self, stack: &mut Stack) -> Plugin {
        let out = Plugin(Rc::new(Plugin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PluginData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PluginRef {
    shared: StackShared,
    base: String,
}

impl Ref for PluginRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PluginRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alias` after provisioning.\nDocker Plugin alias"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_timeout` after provisioning.\nHTTP client timeout to enable the plugin"]
    pub fn enable_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIf `true` the plugin is enabled. Defaults to `true`"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nThe environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_destroy` after provisioning.\nIf true, then the plugin is destroyed forcibly"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_disable` after provisioning.\nIf true, then the plugin is disabled forcibly"]
    pub fn force_disable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_disable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `grant_all_permissions` after provisioning.\nIf true, grant all permissions necessary to run the plugin"]
    pub fn grant_all_permissions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_all_permissions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nDocker Plugin name"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plugin_reference` after provisioning.\nDocker Plugin Reference"]
    pub fn plugin_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugin_reference", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PluginGrantPermissionsEl {
    name: PrimField<String>,
    value: SetField<PrimField<String>>,
}

impl PluginGrantPermissionsEl { }

impl ToListMappable for PluginGrantPermissionsEl {
    type O = BlockAssignable<PluginGrantPermissionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPluginGrantPermissionsEl {
    #[doc= "The name of the permission"]
    pub name: PrimField<String>,
    #[doc= "The value of the permission"]
    pub value: SetField<PrimField<String>>,
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
    shared: StackShared,
    base: String,
}

impl Ref for PluginGrantPermissionsElRef {
    fn new(shared: StackShared, base: String) -> PluginGrantPermissionsElRef {
        PluginGrantPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PluginGrantPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the permission"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the permission"]
    pub fn value(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct PluginDynamic {
    grant_permissions: Option<DynamicBlock<PluginGrantPermissionsEl>>,
}
