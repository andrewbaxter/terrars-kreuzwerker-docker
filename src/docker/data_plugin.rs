use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataPluginData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataPlugin_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPluginData>,
}

#[derive(Clone)]
pub struct DataPlugin(Rc<DataPlugin_>);

impl DataPlugin {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDocker) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `alias`.\nThe alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value."]
    pub fn set_alias(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alias = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\nThe ID of the plugin, which has precedence over the `alias` of both are given"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `alias` after provisioning.\nThe alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value."]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIf `true` the plugin is enabled"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nThe environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `grant_all_permissions` after provisioning.\nIf true, grant all permissions necessary to run the plugin"]
    pub fn grant_all_permissions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_all_permissions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the plugin, which has precedence over the `alias` of both are given"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe plugin name. If the tag is omitted, `:latest` is complemented to the attribute value."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plugin_reference` after provisioning.\nThe Docker Plugin Reference"]
    pub fn plugin_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugin_reference", self.extract_ref()))
    }
}

impl Referable for DataPlugin {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataPlugin { }

impl ToListMappable for DataPlugin {
    type O = ListRef<DataPluginRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataPlugin_ {
    fn extract_datasource_type(&self) -> String {
        "docker_plugin".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPlugin {
    pub tf_id: String,
}

impl BuildDataPlugin {
    pub fn build(self, stack: &mut Stack) -> DataPlugin {
        let out = DataPlugin(Rc::new(DataPlugin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataPluginData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                alias: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPluginRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPluginRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPluginRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `alias` after provisioning.\nThe alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value."]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIf `true` the plugin is enabled"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nThe environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `grant_all_permissions` after provisioning.\nIf true, grant all permissions necessary to run the plugin"]
    pub fn grant_all_permissions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_all_permissions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the plugin, which has precedence over the `alias` of both are given"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe plugin name. If the tag is omitted, `:latest` is complemented to the attribute value."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plugin_reference` after provisioning.\nThe Docker Plugin Reference"]
    pub fn plugin_reference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plugin_reference", self.extract_ref()))
    }
}
