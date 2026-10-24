use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataLogsData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discard_headers: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    follow: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs_list_string_enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_stderr: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_stdout: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    since: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tail: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    until: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLogs_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLogsData>,
}

#[derive(Clone)]
pub struct DataLogs(std::rc::Rc<DataLogs_>);

impl DataLogs {
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

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discard_headers`.\nDiscard headers that docker appends to each log entry"]
    pub fn set_discard_headers(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().discard_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `follow`.\n"]
    pub fn set_follow(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().follow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logs_list_string_enabled`.\nIf true populate computed value `logs_list_string`"]
    pub fn set_logs_list_string_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logs_list_string_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show_stderr`.\n"]
    pub fn set_show_stderr(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_stderr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show_stdout`.\n"]
    pub fn set_show_stdout(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_stdout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `since`.\n"]
    pub fn set_since(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().since = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tail`.\n"]
    pub fn set_tail(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamps`.\n"]
    pub fn set_timestamps(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().timestamps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `until`.\n"]
    pub fn set_until(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().until = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discard_headers` after provisioning.\nDiscard headers that docker appends to each log entry"]
    pub fn discard_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discard_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `follow` after provisioning.\n"]
    pub fn follow(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.follow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logs_list_string` after provisioning.\nList of container logs, each element is a line."]
    pub fn logs_list_string(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_list_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logs_list_string_enabled` after provisioning.\nIf true populate computed value `logs_list_string`"]
    pub fn logs_list_string_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logs_list_string_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker Container"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `show_stderr` after provisioning.\n"]
    pub fn show_stderr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_stderr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `show_stdout` after provisioning.\n"]
    pub fn show_stdout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_stdout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `since` after provisioning.\n"]
    pub fn since(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.since", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tail` after provisioning.\n"]
    pub fn tail(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timestamps` after provisioning.\n"]
    pub fn timestamps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `until` after provisioning.\n"]
    pub fn until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.until", self.extract_ref()))
    }
}

impl terrars::Referable for DataLogs {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLogs { }

impl terrars::ToListMappable for DataLogs {
    type O = terrars::ListRef<DataLogsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLogs_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "docker_logs".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLogs {
    pub tf_id: std::string::String,
    #[doc = "The name of the Docker Container"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLogs {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLogs {
        let out = DataLogs(std::rc::Rc::new(DataLogs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLogsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                details: core::default::Default::default(),
                discard_headers: core::default::Default::default(),
                follow: core::default::Default::default(),
                id: core::default::Default::default(),
                logs_list_string_enabled: core::default::Default::default(),
                name: self.name,
                show_stderr: core::default::Default::default(),
                show_stdout: core::default::Default::default(),
                since: core::default::Default::default(),
                tail: core::default::Default::default(),
                timestamps: core::default::Default::default(),
                until: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLogsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discard_headers` after provisioning.\nDiscard headers that docker appends to each log entry"]
    pub fn discard_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discard_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `follow` after provisioning.\n"]
    pub fn follow(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.follow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logs_list_string` after provisioning.\nList of container logs, each element is a line."]
    pub fn logs_list_string(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_list_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logs_list_string_enabled` after provisioning.\nIf true populate computed value `logs_list_string`"]
    pub fn logs_list_string_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logs_list_string_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker Container"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `show_stderr` after provisioning.\n"]
    pub fn show_stderr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_stderr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `show_stdout` after provisioning.\n"]
    pub fn show_stdout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_stdout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `since` after provisioning.\n"]
    pub fn since(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.since", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tail` after provisioning.\n"]
    pub fn tail(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timestamps` after provisioning.\n"]
    pub fn timestamps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `until` after provisioning.\n"]
    pub fn until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.until", self.extract_ref()))
    }
}
