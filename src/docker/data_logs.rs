use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataLogsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discard_headers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follow: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logs_list_string_enabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_stderr: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_stdout: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tail: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<PrimField<String>>,
}

struct DataLogs_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLogsData>,
}

#[derive(Clone)]
pub struct DataLogs(Rc<DataLogs_>);

impl DataLogs {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDocker) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `details`.\n"]
    pub fn set_details(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().details = Some(v.into());
        self
    }

    #[doc= "Set the field `discard_headers`.\nDiscard headers that docker appends to each log entry"]
    pub fn set_discard_headers(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().discard_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `follow`.\n"]
    pub fn set_follow(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().follow = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `logs_list_string_enabled`.\nIf true populate computed value `logs_list_string`"]
    pub fn set_logs_list_string_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logs_list_string_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `show_stderr`.\n"]
    pub fn set_show_stderr(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_stderr = Some(v.into());
        self
    }

    #[doc= "Set the field `show_stdout`.\n"]
    pub fn set_show_stdout(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_stdout = Some(v.into());
        self
    }

    #[doc= "Set the field `since`.\n"]
    pub fn set_since(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().since = Some(v.into());
        self
    }

    #[doc= "Set the field `tail`.\n"]
    pub fn set_tail(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tail = Some(v.into());
        self
    }

    #[doc= "Set the field `timestamps`.\n"]
    pub fn set_timestamps(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().timestamps = Some(v.into());
        self
    }

    #[doc= "Set the field `until`.\n"]
    pub fn set_until(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().until = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discard_headers` after provisioning.\nDiscard headers that docker appends to each log entry"]
    pub fn discard_headers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.discard_headers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `follow` after provisioning.\n"]
    pub fn follow(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.follow", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logs_list_string` after provisioning.\nList of container logs, each element is a line."]
    pub fn logs_list_string(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.logs_list_string", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logs_list_string_enabled` after provisioning.\nIf true populate computed value `logs_list_string`"]
    pub fn logs_list_string_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.logs_list_string_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker Container"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `show_stderr` after provisioning.\n"]
    pub fn show_stderr(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show_stderr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `show_stdout` after provisioning.\n"]
    pub fn show_stdout(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show_stdout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `since` after provisioning.\n"]
    pub fn since(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.since", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tail` after provisioning.\n"]
    pub fn tail(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tail", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timestamps` after provisioning.\n"]
    pub fn timestamps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.timestamps", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `until` after provisioning.\n"]
    pub fn until(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.until", self.extract_ref()))
    }
}

impl Datasource for DataLogs {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataLogs {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataLogs {
    type O = ListRef<DataLogsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataLogs_ {
    fn extract_datasource_type(&self) -> String {
        "docker_logs".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLogs {
    pub tf_id: String,
    #[doc= "The name of the Docker Container"]
    pub name: PrimField<String>,
}

impl BuildDataLogs {
    pub fn build(self, stack: &mut Stack) -> DataLogs {
        let out = DataLogs(Rc::new(DataLogs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLogsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for DataLogsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLogsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discard_headers` after provisioning.\nDiscard headers that docker appends to each log entry"]
    pub fn discard_headers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.discard_headers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `follow` after provisioning.\n"]
    pub fn follow(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.follow", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logs_list_string` after provisioning.\nList of container logs, each element is a line."]
    pub fn logs_list_string(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.logs_list_string", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logs_list_string_enabled` after provisioning.\nIf true populate computed value `logs_list_string`"]
    pub fn logs_list_string_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.logs_list_string_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker Container"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `show_stderr` after provisioning.\n"]
    pub fn show_stderr(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show_stderr", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `show_stdout` after provisioning.\n"]
    pub fn show_stdout(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show_stdout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `since` after provisioning.\n"]
    pub fn since(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.since", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tail` after provisioning.\n"]
    pub fn tail(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tail", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timestamps` after provisioning.\n"]
    pub fn timestamps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.timestamps", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `until` after provisioning.\n"]
    pub fn until(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.until", self.extract_ref()))
    }
}
