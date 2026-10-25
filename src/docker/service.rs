#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct ServiceData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth: std::option::Option<std::vec::Vec<Service_Auth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    converge_config: std::option::Option<std::vec::Vec<Service_ConvergeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_spec: std::option::Option<std::vec::Vec<Service_EndpointSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<Service_Labels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<std::vec::Vec<Service_Mode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_config: std::option::Option<std::vec::Vec<Service_RollbackConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_spec: std::option::Option<std::vec::Vec<Service_TaskSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_config: std::option::Option<std::vec::Vec<Service_UpdateConfig>>,
    dynamic: ServiceDynamic,
}

struct Service_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ServiceData_>,
}

#[derive(Clone)]
pub struct Service(std::rc::Rc<Service_>);

impl Service {
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

    #[doc = "Set the field `auth`.\n"]
    pub fn set_auth(self, v: impl std::convert::Into<terrars::BlockAssignable<Service_Auth>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `converge_config`.\n"]
    pub fn set_converge_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_ConvergeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().converge_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.converge_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_spec`.\n"]
    pub fn set_endpoint_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<Service_EndpointSpec>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl std::convert::Into<terrars::BlockAssignable<Service_Labels>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(self, v: impl std::convert::Into<terrars::BlockAssignable<Service_Mode>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rollback_config`.\n"]
    pub fn set_rollback_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_RollbackConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollback_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollback_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `task_spec`.\n"]
    pub fn set_task_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().task_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.task_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `update_config`.\n"]
    pub fn set_update_config(self, v: impl std::convert::Into<terrars::BlockAssignable<Service_UpdateConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the service"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> terrars::ListRef<Service_AuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `converge_config` after provisioning.\n"]
    pub fn converge_config(&self) -> terrars::ListRef<Service_ConvergeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.converge_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_spec` after provisioning.\n"]
    pub fn endpoint_spec(&self) -> terrars::ListRef<Service_EndpointSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::ListRef<Service_ModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollback_config` after provisioning.\n"]
    pub fn rollback_config(&self) -> terrars::ListRef<Service_RollbackConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_spec` after provisioning.\n"]
    pub fn task_spec(&self) -> terrars::ListRef<Service_TaskSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<Service_UpdateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }
}

impl terrars::Referable for Service {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Service { }

impl terrars::ToListMappable for Service {
    type O = terrars::ListRef<ServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Service_ {
    fn extract_resource_type(&self) -> std::string::String {
        "docker_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildService {
    pub tf_id: std::string::String,
    #[doc = "Name of the service"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildService {
    pub fn build(self, stack: &mut terrars::Stack) -> Service {
        let out = Service(std::rc::Rc::new(Service_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                auth: core::default::Default::default(),
                converge_config: core::default::Default::default(),
                endpoint_spec: core::default::Default::default(),
                labels: core::default::Default::default(),
                mode: core::default::Default::default(),
                rollback_config: core::default::Default::default(),
                task_spec: core::default::Default::default(),
                update_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServiceRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the service"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> terrars::ListRef<Service_AuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `converge_config` after provisioning.\n"]
    pub fn converge_config(&self) -> terrars::ListRef<Service_ConvergeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.converge_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_spec` after provisioning.\n"]
    pub fn endpoint_spec(&self) -> terrars::ListRef<Service_EndpointSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::ListRef<Service_ModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollback_config` after provisioning.\n"]
    pub fn rollback_config(&self) -> terrars::ListRef<Service_RollbackConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_spec` after provisioning.\n"]
    pub fn task_spec(&self) -> terrars::ListRef<Service_TaskSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<Service_UpdateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ServiceDynamic {
    auth: std::option::Option<terrars::DynamicBlock<Service_Auth>>,
    converge_config: std::option::Option<terrars::DynamicBlock<Service_ConvergeConfig>>,
    endpoint_spec: std::option::Option<terrars::DynamicBlock<Service_EndpointSpec>>,
    labels: std::option::Option<terrars::DynamicBlock<Service_Labels>>,
    mode: std::option::Option<terrars::DynamicBlock<Service_Mode>>,
    rollback_config: std::option::Option<terrars::DynamicBlock<Service_RollbackConfig>>,
    task_spec: std::option::Option<terrars::DynamicBlock<Service_TaskSpec>>,
    update_config: std::option::Option<terrars::DynamicBlock<Service_UpdateConfig>>,
}

#[derive(Serialize)]
pub struct Service_Auth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    server_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_Auth {
    #[doc = "Set the field `password`.\nThe password"]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_Auth {
    type O = terrars::BlockAssignable<Service_Auth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_Auth {
    #[doc = "The address of the server for the authentication"]
    pub server_address: terrars::PrimField<std::string::String>,
}

impl BuildService_Auth {
    pub fn build(self) -> Service_Auth {
        Service_Auth {
            password: core::default::Default::default(),
            server_address: self.server_address,
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_AuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_AuthRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_AuthRef {
        Service_AuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_AuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_address` after provisioning.\nThe address of the server for the authentication"]
    pub fn server_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_address", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_ConvergeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_ConvergeConfig {
    #[doc =
        "Set the field `delay`.\nThe interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`."]
    pub fn set_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nThe timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_ConvergeConfig {
    type O = terrars::BlockAssignable<Service_ConvergeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_ConvergeConfig {}

impl BuildService_ConvergeConfig {
    pub fn build(self) -> Service_ConvergeConfig {
        Service_ConvergeConfig {
            delay: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_ConvergeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_ConvergeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_ConvergeConfigRef {
        Service_ConvergeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_ConvergeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delay` after provisioning.\nThe interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`."]
    pub fn delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`"]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_EndpointSpec_Ports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    published_port: std::option::Option<terrars::PrimField<f64>>,
    target_port: terrars::PrimField<f64>,
}

impl Service_EndpointSpec_Ports {
    #[doc = "Set the field `name`.\nA random name for the port"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\nRepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`."]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `publish_mode`.\nRepresents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`."]
    pub fn set_publish_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.publish_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `published_port`.\nThe port on the swarm hosts"]
    pub fn set_published_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.published_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_EndpointSpec_Ports {
    type O = terrars::BlockAssignable<Service_EndpointSpec_Ports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_EndpointSpec_Ports {
    #[doc = "The port inside the container"]
    pub target_port: terrars::PrimField<f64>,
}

impl BuildService_EndpointSpec_Ports {
    pub fn build(self) -> Service_EndpointSpec_Ports {
        Service_EndpointSpec_Ports {
            name: core::default::Default::default(),
            protocol: core::default::Default::default(),
            publish_mode: core::default::Default::default(),
            published_port: core::default::Default::default(),
            target_port: self.target_port,
        }
    }
}

#[allow(dead_code)]
pub struct Service_EndpointSpec_PortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_EndpointSpec_PortsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_EndpointSpec_PortsRef {
        Service_EndpointSpec_PortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_EndpointSpec_PortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA random name for the port"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nRepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `publish_mode` after provisioning.\nRepresents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`."]
    pub fn publish_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `published_port` after provisioning.\nThe port on the swarm hosts"]
    pub fn published_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.published_port", self.base))
    }

    #[doc = "Get a reference to the value of field `target_port` after provisioning.\nThe port inside the container"]
    pub fn target_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_EndpointSpecDynamic {
    ports: std::option::Option<terrars::DynamicBlock<Service_EndpointSpec_Ports>>,
}

#[derive(Serialize)]
pub struct Service_EndpointSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<Service_EndpointSpec_Ports>>,
    dynamic: Service_EndpointSpecDynamic,
}

impl Service_EndpointSpec {
    #[doc = "Set the field `mode`.\nThe mode of resolution to use for internal load balancing between tasks"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_EndpointSpec_Ports>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ports = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_EndpointSpec {
    type O = terrars::BlockAssignable<Service_EndpointSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_EndpointSpec {}

impl BuildService_EndpointSpec {
    pub fn build(self) -> Service_EndpointSpec {
        Service_EndpointSpec {
            mode: core::default::Default::default(),
            ports: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_EndpointSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_EndpointSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_EndpointSpecRef {
        Service_EndpointSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_EndpointSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode of resolution to use for internal load balancing between tasks"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<Service_EndpointSpec_PortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_Labels {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Service_Labels { }

impl terrars::ToListMappable for Service_Labels {
    type O = terrars::BlockAssignable<Service_Labels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_Labels {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildService_Labels {
    pub fn build(self) -> Service_Labels {
        Service_Labels {
            label: self.label,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Service_LabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_LabelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_LabelsRef {
        Service_LabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_LabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\nName of the label"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the label"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_Mode_Replicated {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::PrimField<f64>>,
}

impl Service_Mode_Replicated {
    #[doc = "Set the field `replicas`.\nThe amount of replicas of the service. Defaults to `1`"]
    pub fn set_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_Mode_Replicated {
    type O = terrars::BlockAssignable<Service_Mode_Replicated>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_Mode_Replicated {}

impl BuildService_Mode_Replicated {
    pub fn build(self) -> Service_Mode_Replicated {
        Service_Mode_Replicated { replicas: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Service_Mode_ReplicatedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_Mode_ReplicatedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_Mode_ReplicatedRef {
        Service_Mode_ReplicatedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_Mode_ReplicatedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `replicas` after provisioning.\nThe amount of replicas of the service. Defaults to `1`"]
    pub fn replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_ModeDynamic {
    replicated: std::option::Option<terrars::DynamicBlock<Service_Mode_Replicated>>,
}

#[derive(Serialize)]
pub struct Service_Mode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicated: std::option::Option<std::vec::Vec<Service_Mode_Replicated>>,
    dynamic: Service_ModeDynamic,
}

impl Service_Mode {
    #[doc = "Set the field `global`.\nThe global service mode. Defaults to `false`"]
    pub fn set_global(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.global = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicated`.\n"]
    pub fn set_replicated(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_Mode_Replicated>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replicated = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replicated = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_Mode {
    type O = terrars::BlockAssignable<Service_Mode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_Mode {}

impl BuildService_Mode {
    pub fn build(self) -> Service_Mode {
        Service_Mode {
            global: core::default::Default::default(),
            replicated: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_ModeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_ModeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_ModeRef {
        Service_ModeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_ModeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `global` after provisioning.\nThe global service mode. Defaults to `false`"]
    pub fn global(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global", self.base))
    }

    #[doc = "Get a reference to the value of field `replicated` after provisioning.\n"]
    pub fn replicated(&self) -> terrars::ListRef<Service_Mode_ReplicatedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicated", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_RollbackConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_failure_ratio: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism: std::option::Option<terrars::PrimField<f64>>,
}

impl Service_RollbackConfig {
    #[doc = "Set the field `delay`.\nDelay between task rollbacks (ns|us|ms|s|m|h). Defaults to `0s`."]
    pub fn set_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_action`.\nAction on rollback failure: pause | continue. Defaults to `pause`."]
    pub fn set_failure_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failure_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_failure_ratio`.\nFailure rate to tolerate during a rollback. Defaults to `0.0`."]
    pub fn set_max_failure_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_failure_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `monitor`.\nDuration after each task rollback to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`."]
    pub fn set_monitor(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.monitor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nRollback order: either 'stop-first' or 'start-first'. Defaults to `stop-first`."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelism`.\nMaximum number of tasks to be rollbacked in one iteration. Defaults to `1`"]
    pub fn set_parallelism(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallelism = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_RollbackConfig {
    type O = terrars::BlockAssignable<Service_RollbackConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_RollbackConfig {}

impl BuildService_RollbackConfig {
    pub fn build(self) -> Service_RollbackConfig {
        Service_RollbackConfig {
            delay: core::default::Default::default(),
            failure_action: core::default::Default::default(),
            max_failure_ratio: core::default::Default::default(),
            monitor: core::default::Default::default(),
            order: core::default::Default::default(),
            parallelism: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_RollbackConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_RollbackConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_RollbackConfigRef {
        Service_RollbackConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_RollbackConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delay` after provisioning.\nDelay between task rollbacks (ns|us|ms|s|m|h). Defaults to `0s`."]
    pub fn delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `failure_action` after provisioning.\nAction on rollback failure: pause | continue. Defaults to `pause`."]
    pub fn failure_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_failure_ratio` after provisioning.\nFailure rate to tolerate during a rollback. Defaults to `0.0`."]
    pub fn max_failure_ratio(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_failure_ratio", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitor` after provisioning.\nDuration after each task rollback to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`."]
    pub fn monitor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nRollback order: either 'stop-first' or 'start-first'. Defaults to `stop-first`."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parallelism` after provisioning.\nMaximum number of tasks to be rollbacked in one iteration. Defaults to `1`"]
    pub fn parallelism(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Configs {
    config_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_gid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_mode: std::option::Option<terrars::PrimField<f64>>,
    file_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_TaskSpec_ContainerSpec_Configs {
    #[doc =
        "Set the field `config_name`.\nName of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID"]
    pub fn set_config_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.config_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_gid`.\nRepresents the file GID. Defaults to `0`."]
    pub fn set_file_gid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_gid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_mode`.\nRepresents represents the FileMode of the file. Defaults to `0o444`."]
    pub fn set_file_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.file_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_uid`.\nRepresents the file UID. Defaults to `0`."]
    pub fn set_file_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Configs {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Configs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Configs {
    #[doc = "ID of the specific config that we're referencing"]
    pub config_id: terrars::PrimField<std::string::String>,
    #[doc = "Represents the final filename in the filesystem"]
    pub file_name: terrars::PrimField<std::string::String>,
}

impl BuildService_TaskSpec_ContainerSpec_Configs {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Configs {
        Service_TaskSpec_ContainerSpec_Configs {
            config_id: self.config_id,
            config_name: core::default::Default::default(),
            file_gid: core::default::Default::default(),
            file_mode: core::default::Default::default(),
            file_name: self.file_name,
            file_uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_ConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_ConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ContainerSpec_ConfigsRef {
        Service_TaskSpec_ContainerSpec_ConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_ConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config_id` after provisioning.\nID of the specific config that we're referencing"]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `config_name` after provisioning.\nName of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID"]
    pub fn config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_gid` after provisioning.\nRepresents the file GID. Defaults to `0`."]
    pub fn file_gid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_gid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_mode` after provisioning.\nRepresents represents the FileMode of the file. Defaults to `0o444`."]
    pub fn file_mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_name` after provisioning.\nRepresents the final filename in the filesystem"]
    pub fn file_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uid` after provisioning.\nRepresents the file UID. Defaults to `0`."]
    pub fn file_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_DnsConfig {
    nameservers: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Service_TaskSpec_ContainerSpec_DnsConfig {
    #[doc =
        "Set the field `options`.\nA list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.)"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\nA search list for host-name lookup"]
    pub fn set_search(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_DnsConfig {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_DnsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_DnsConfig {
    #[doc = "The IP addresses of the name servers"]
    pub nameservers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildService_TaskSpec_ContainerSpec_DnsConfig {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_DnsConfig {
        Service_TaskSpec_ContainerSpec_DnsConfig {
            nameservers: self.nameservers,
            options: core::default::Default::default(),
            search: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_DnsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_DnsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ContainerSpec_DnsConfigRef {
        Service_TaskSpec_ContainerSpec_DnsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_DnsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nameservers` after provisioning.\nThe IP addresses of the name servers"]
    pub fn nameservers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nameservers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `options` after provisioning.\nA list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.)"]
    pub fn options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\nA search list for host-name lookup"]
    pub fn search(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Healthcheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_TaskSpec_ContainerSpec_Healthcheck {
    #[doc = "Set the field `interval`.\nTime between running the check (ms|s|m|h). Defaults to `0s`."]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retries`.\nConsecutive failures needed to report unhealthy. Defaults to `0`"]
    pub fn set_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_period`.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn set_start_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test`.\nThe test to perform as list"]
    pub fn set_test(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.test = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Healthcheck {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Healthcheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Healthcheck {}

impl BuildService_TaskSpec_ContainerSpec_Healthcheck {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Healthcheck {
        Service_TaskSpec_ContainerSpec_Healthcheck {
            interval: core::default::Default::default(),
            retries: core::default::Default::default(),
            start_period: core::default::Default::default(),
            test: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_HealthcheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_HealthcheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ContainerSpec_HealthcheckRef {
        Service_TaskSpec_ContainerSpec_HealthcheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_HealthcheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nTime between running the check (ms|s|m|h). Defaults to `0s`."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nConsecutive failures needed to report unhealthy. Defaults to `0`"]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_period` after provisioning.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn start_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_period", self.base))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\nThe test to perform as list"]
    pub fn test(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Hosts {
    host: terrars::PrimField<std::string::String>,
    ip: terrars::PrimField<std::string::String>,
}

impl Service_TaskSpec_ContainerSpec_Hosts { }

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Hosts {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Hosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Hosts {
    #[doc = "The name of the host"]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "The ip of the host"]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildService_TaskSpec_ContainerSpec_Hosts {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Hosts {
        Service_TaskSpec_ContainerSpec_Hosts {
            host: self.host,
            ip: self.ip,
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_HostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_HostsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ContainerSpec_HostsRef {
        Service_TaskSpec_ContainerSpec_HostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_HostsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe name of the host"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\nThe ip of the host"]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Mounts_BindOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_TaskSpec_ContainerSpec_Mounts_BindOptions {
    #[doc =
        "Set the field `propagation`.\nBind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`"]
    pub fn set_propagation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.propagation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Mounts_BindOptions {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Mounts_BindOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Mounts_BindOptions {}

impl BuildService_TaskSpec_ContainerSpec_Mounts_BindOptions {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Mounts_BindOptions {
        Service_TaskSpec_ContainerSpec_Mounts_BindOptions { propagation: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_Mounts_BindOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_Mounts_BindOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Service_TaskSpec_ContainerSpec_Mounts_BindOptionsRef {
        Service_TaskSpec_ContainerSpec_Mounts_BindOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_Mounts_BindOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `propagation` after provisioning.\nBind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`"]
    pub fn propagation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagation", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_bytes: std::option::Option<terrars::PrimField<f64>>,
}

impl Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions {
    #[doc = "Set the field `mode`.\nThe permission mode for the tmpfs mount in an integer"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_bytes`.\nThe size for the tmpfs mount in bytes"]
    pub fn set_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Mounts_TmpfsOptions {}

impl BuildService_TaskSpec_ContainerSpec_Mounts_TmpfsOptions {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions {
        Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions {
            mode: core::default::Default::default(),
            size_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptionsRef {
        Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe permission mode for the tmpfs mount in an integer"]
    pub fn mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_bytes` after provisioning.\nThe size for the tmpfs mount in bytes"]
    pub fn size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_bytes", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_TaskSpec_ContainerSpec_Mounts_VolumeOptionsDynamic {
    labels: std::option::Option<terrars::DynamicBlock<Service_Labels>>,
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_copy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<Service_Labels>>,
    dynamic: Service_TaskSpec_ContainerSpec_Mounts_VolumeOptionsDynamic,
}

impl Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions {
    #[doc = "Set the field `driver_name`.\nName of the driver to use to create the volume"]
    pub fn set_driver_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.driver_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver_options`.\nkey/value map of driver specific options"]
    pub fn set_driver_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_copy`.\nPopulate volume with data from the target"]
    pub fn set_no_copy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_copy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl std::convert::Into<terrars::BlockAssignable<Service_Labels>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Mounts_VolumeOptions {}

impl BuildService_TaskSpec_ContainerSpec_Mounts_VolumeOptions {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions {
        Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions {
            driver_name: core::default::Default::default(),
            driver_options: core::default::Default::default(),
            no_copy: core::default::Default::default(),
            labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_Mounts_VolumeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_Mounts_VolumeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Service_TaskSpec_ContainerSpec_Mounts_VolumeOptionsRef {
        Service_TaskSpec_ContainerSpec_Mounts_VolumeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_Mounts_VolumeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_name` after provisioning.\nName of the driver to use to create the volume"]
    pub fn driver_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `driver_options` after provisioning.\nkey/value map of driver specific options"]
    pub fn driver_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_copy` after provisioning.\nPopulate volume with data from the target"]
    pub fn no_copy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_copy", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_TaskSpec_ContainerSpec_MountsDynamic {
    bind_options: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Mounts_BindOptions>>,
    tmpfs_options: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions>>,
    volume_options: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions>>,
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Mounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bind_options: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Mounts_BindOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tmpfs_options: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_options: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions>>,
    dynamic: Service_TaskSpec_ContainerSpec_MountsDynamic,
}

impl Service_TaskSpec_ContainerSpec_Mounts {
    #[doc = "Set the field `read_only`.\nWhether the mount should be read-only"]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\nMount source (e.g. a volume name, a host path)"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bind_options`.\n"]
    pub fn set_bind_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Mounts_BindOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bind_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bind_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tmpfs_options`.\n"]
    pub fn set_tmpfs_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tmpfs_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tmpfs_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volume_options`.\n"]
    pub fn set_volume_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Mounts_VolumeOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volume_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Mounts {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Mounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Mounts {
    #[doc = "Container path"]
    pub target: terrars::PrimField<std::string::String>,
    #[doc = "The mount type"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildService_TaskSpec_ContainerSpec_Mounts {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Mounts {
        Service_TaskSpec_ContainerSpec_Mounts {
            read_only: core::default::Default::default(),
            source: core::default::Default::default(),
            target: self.target,
            type_: self.type_,
            bind_options: core::default::Default::default(),
            tmpfs_options: core::default::Default::default(),
            volume_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_MountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_MountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ContainerSpec_MountsRef {
        Service_TaskSpec_ContainerSpec_MountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_MountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nWhether the mount should be read-only"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nMount source (e.g. a volume name, a host path)"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\nContainer path"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe mount type"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `bind_options` after provisioning.\n"]
    pub fn bind_options(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpec_Mounts_BindOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bind_options", self.base))
    }

    #[doc = "Get a reference to the value of field `tmpfs_options` after provisioning.\n"]
    pub fn tmpfs_options(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpec_Mounts_TmpfsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tmpfs_options", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_options` after provisioning.\n"]
    pub fn volume_options(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpec_Mounts_VolumeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_options", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec {
    #[doc = "Set the field `file`.\nLoad credential spec from this file"]
    pub fn set_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry`.\nLoad credential spec from this value in the Windows registry"]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Privileges_CredentialSpec {}

impl BuildService_TaskSpec_ContainerSpec_Privileges_CredentialSpec {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec {
        Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec {
            file: core::default::Default::default(),
            registry: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_Privileges_CredentialSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_Privileges_CredentialSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Service_TaskSpec_ContainerSpec_Privileges_CredentialSpecRef {
        Service_TaskSpec_ContainerSpec_Privileges_CredentialSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_Privileges_CredentialSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\nLoad credential spec from this file"]
    pub fn file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `registry` after provisioning.\nLoad credential spec from this value in the Windows registry"]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext {
    #[doc = "Set the field `disable`.\nDisable SELinux"]
    pub fn set_disable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level`.\nSELinux level label"]
    pub fn set_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\nSELinux role label"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nSELinux type label"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nSELinux user label"]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Privileges_SeLinuxContext {}

impl BuildService_TaskSpec_ContainerSpec_Privileges_SeLinuxContext {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext {
        Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext {
            disable: core::default::Default::default(),
            level: core::default::Default::default(),
            role: core::default::Default::default(),
            type_: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContextRef {
        Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable` after provisioning.\nDisable SELinux"]
    pub fn disable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\nSELinux level label"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\nSELinux role label"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nSELinux type label"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nSELinux user label"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_TaskSpec_ContainerSpec_PrivilegesDynamic {
    credential_spec: std
    ::option
    ::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec>>,
    se_linux_context: std
    ::option
    ::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext>>,
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Privileges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_spec: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    se_linux_context: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext>>,
    dynamic: Service_TaskSpec_ContainerSpec_PrivilegesDynamic,
}

impl Service_TaskSpec_ContainerSpec_Privileges {
    #[doc = "Set the field `credential_spec`.\n"]
    pub fn set_credential_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Privileges_CredentialSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.credential_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.credential_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `se_linux_context`.\n"]
    pub fn set_se_linux_context(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContext>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.se_linux_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.se_linux_context = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Privileges {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Privileges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Privileges {}

impl BuildService_TaskSpec_ContainerSpec_Privileges {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Privileges {
        Service_TaskSpec_ContainerSpec_Privileges {
            credential_spec: core::default::Default::default(),
            se_linux_context: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_PrivilegesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_PrivilegesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ContainerSpec_PrivilegesRef {
        Service_TaskSpec_ContainerSpec_PrivilegesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_PrivilegesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credential_spec` after provisioning.\n"]
    pub fn credential_spec(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpec_Privileges_CredentialSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credential_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `se_linux_context` after provisioning.\n"]
    pub fn se_linux_context(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpec_Privileges_SeLinuxContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.se_linux_context", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec_Secrets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_gid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_mode: std::option::Option<terrars::PrimField<f64>>,
    file_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uid: std::option::Option<terrars::PrimField<std::string::String>>,
    secret_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_TaskSpec_ContainerSpec_Secrets {
    #[doc = "Set the field `file_gid`.\nRepresents the file GID. Defaults to `0`"]
    pub fn set_file_gid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_gid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_mode`.\nRepresents represents the FileMode of the file. Defaults to `0o444`"]
    pub fn set_file_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.file_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_uid`.\nRepresents the file UID. Defaults to `0`"]
    pub fn set_file_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_uid = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_name`.\nName of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID"]
    pub fn set_secret_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec_Secrets {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Secrets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec_Secrets {
    #[doc = "Represents the final filename in the filesystem"]
    pub file_name: terrars::PrimField<std::string::String>,
    #[doc = "ID of the specific secret that we're referencing"]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildService_TaskSpec_ContainerSpec_Secrets {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec_Secrets {
        Service_TaskSpec_ContainerSpec_Secrets {
            file_gid: core::default::Default::default(),
            file_mode: core::default::Default::default(),
            file_name: self.file_name,
            file_uid: core::default::Default::default(),
            secret_id: self.secret_id,
            secret_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpec_SecretsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpec_SecretsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ContainerSpec_SecretsRef {
        Service_TaskSpec_ContainerSpec_SecretsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpec_SecretsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_gid` after provisioning.\nRepresents the file GID. Defaults to `0`"]
    pub fn file_gid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_gid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_mode` after provisioning.\nRepresents represents the FileMode of the file. Defaults to `0o444`"]
    pub fn file_mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_name` after provisioning.\nRepresents the final filename in the filesystem"]
    pub fn file_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uid` after provisioning.\nRepresents the file UID. Defaults to `0`"]
    pub fn file_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_uid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_id` after provisioning.\nID of the specific secret that we're referencing"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_name` after provisioning.\nName of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_TaskSpec_ContainerSpecDynamic {
    configs: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Configs>>,
    dns_config: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_DnsConfig>>,
    healthcheck: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Healthcheck>>,
    hosts: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Hosts>>,
    labels: std::option::Option<terrars::DynamicBlock<Service_Labels>>,
    mounts: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Mounts>>,
    privileges: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Privileges>>,
    secrets: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec_Secrets>>,
}

#[derive(Serialize)]
pub struct Service_TaskSpec_ContainerSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cap_add: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cap_drop: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    isolation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_grace_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_signal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sysctl: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configs: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Configs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_config: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_DnsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthcheck: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Healthcheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Hosts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<Service_Labels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mounts: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Mounts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileges: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Privileges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec_Secrets>>,
    dynamic: Service_TaskSpec_ContainerSpecDynamic,
}

impl Service_TaskSpec_ContainerSpec {
    #[doc = "Set the field `args`.\nArguments to the command"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cap_add`.\nList of Linux capabilities to add to the container"]
    pub fn set_cap_add(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cap_add = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cap_drop`.\nList of Linux capabilities to drop from the container"]
    pub fn set_cap_drop(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cap_drop = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `command`.\nThe command/entrypoint to be run in the image. According to the [docker cli](https://github.com/docker/cli/blob/v20.10.7/cli/command/service/opts.go#L705) the override of the entrypoint is also passed to the `command` property and there is no `entrypoint` attribute in the `ContainerSpec` of the service."]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dir`.\nThe working directory for commands to run in"]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\nA list of environment variables in the form VAR=\"value\""]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `groups`.\nA list of additional groups that the container process will run as"]
    pub fn set_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nThe hostname to use for the container, as a valid RFC 1123 hostname"]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `init`.\nConfigured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults."]
    pub fn set_init(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.init = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `isolation`.\nIsolation technology of the containers running the service. (Windows only). Defaults to `default`."]
    pub fn set_isolation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\nMount the container's root filesystem as read only"]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stop_grace_period`.\nAmount of time to wait for the container to terminate before forcefully removing it (ms|s|m|h). If not specified or '0s' the destroy will not check if all tasks/containers of the service terminate."]
    pub fn set_stop_grace_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stop_grace_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_signal`.\nSignal to stop the container"]
    pub fn set_stop_signal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stop_signal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sysctl`.\nSysctls config (Linux only)"]
    pub fn set_sysctl(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.sysctl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nThe user inside the container"]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configs`.\n"]
    pub fn set_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Configs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns_config`.\n"]
    pub fn set_dns_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_DnsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `healthcheck`.\n"]
    pub fn set_healthcheck(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Healthcheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.healthcheck = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.healthcheck = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Hosts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hosts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hosts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl std::convert::Into<terrars::BlockAssignable<Service_Labels>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mounts`.\n"]
    pub fn set_mounts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Mounts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mounts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `privileges`.\n"]
    pub fn set_privileges(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Privileges>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.privileges = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.privileges = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secrets`.\n"]
    pub fn set_secrets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec_Secrets>>,
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
}

impl terrars::ToListMappable for Service_TaskSpec_ContainerSpec {
    type O = terrars::BlockAssignable<Service_TaskSpec_ContainerSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_ContainerSpec {
    #[doc =
        "The image name to use for the containers of the service, like `nginx:1.17.6`. Also use the data-source or resource of `docker_image` with the `repo_digest` or `docker_registry_image` with the `name` attribute for this, as shown in the examples."]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildService_TaskSpec_ContainerSpec {
    pub fn build(self) -> Service_TaskSpec_ContainerSpec {
        Service_TaskSpec_ContainerSpec {
            args: core::default::Default::default(),
            cap_add: core::default::Default::default(),
            cap_drop: core::default::Default::default(),
            command: core::default::Default::default(),
            dir: core::default::Default::default(),
            env: core::default::Default::default(),
            groups: core::default::Default::default(),
            hostname: core::default::Default::default(),
            image: self.image,
            init: core::default::Default::default(),
            isolation: core::default::Default::default(),
            read_only: core::default::Default::default(),
            stop_grace_period: core::default::Default::default(),
            stop_signal: core::default::Default::default(),
            sysctl: core::default::Default::default(),
            user: core::default::Default::default(),
            configs: core::default::Default::default(),
            dns_config: core::default::Default::default(),
            healthcheck: core::default::Default::default(),
            hosts: core::default::Default::default(),
            labels: core::default::Default::default(),
            mounts: core::default::Default::default(),
            privileges: core::default::Default::default(),
            secrets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ContainerSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ContainerSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ContainerSpecRef {
        Service_TaskSpec_ContainerSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ContainerSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\nArguments to the command"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cap_add` after provisioning.\nList of Linux capabilities to add to the container"]
    pub fn cap_add(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cap_add", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cap_drop` after provisioning.\nList of Linux capabilities to drop from the container"]
    pub fn cap_drop(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cap_drop", self.base))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nThe command/entrypoint to be run in the image. According to the [docker cli](https://github.com/docker/cli/blob/v20.10.7/cli/command/service/opts.go#L705) the override of the entrypoint is also passed to the `command` property and there is no `entrypoint` attribute in the `ContainerSpec` of the service."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dir` after provisioning.\nThe working directory for commands to run in"]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nA list of environment variables in the form VAR=\"value\""]
    pub fn env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `groups` after provisioning.\nA list of additional groups that the container process will run as"]
    pub fn groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe hostname to use for the container, as a valid RFC 1123 hostname"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nThe image name to use for the containers of the service, like `nginx:1.17.6`. Also use the data-source or resource of `docker_image` with the `repo_digest` or `docker_registry_image` with the `name` attribute for this, as shown in the examples."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `init` after provisioning.\nConfigured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults."]
    pub fn init(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.init", self.base))
    }

    #[doc =
        "Get a reference to the value of field `isolation` after provisioning.\nIsolation technology of the containers running the service. (Windows only). Defaults to `default`."]
    pub fn isolation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nMount the container's root filesystem as read only"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stop_grace_period` after provisioning.\nAmount of time to wait for the container to terminate before forcefully removing it (ms|s|m|h). If not specified or '0s' the destroy will not check if all tasks/containers of the service terminate."]
    pub fn stop_grace_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_grace_period", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to stop the container"]
    pub fn stop_signal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.base))
    }

    #[doc = "Get a reference to the value of field `sysctl` after provisioning.\nSysctls config (Linux only)"]
    pub fn sysctl(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.sysctl", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nThe user inside the container"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpec_DnsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.base))
    }

    #[doc = "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpec_HealthcheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.base))
    }

    #[doc = "Get a reference to the value of field `privileges` after provisioning.\n"]
    pub fn privileges(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpec_PrivilegesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.privileges", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_LogDriver {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl Service_TaskSpec_LogDriver {
    #[doc = "Set the field `options`.\nThe options for the logging driver"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_LogDriver {
    type O = terrars::BlockAssignable<Service_TaskSpec_LogDriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_LogDriver {
    #[doc = "The logging driver to use"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildService_TaskSpec_LogDriver {
    pub fn build(self) -> Service_TaskSpec_LogDriver {
        Service_TaskSpec_LogDriver {
            name: self.name,
            options: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_LogDriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_LogDriverRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_LogDriverRef {
        Service_TaskSpec_LogDriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_LogDriverRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe logging driver to use"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\nThe options for the logging driver"]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_NetworksAdvanced {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aliases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_opts: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_TaskSpec_NetworksAdvanced {
    #[doc = "Set the field `aliases`.\nThe network aliases of the container in the specific network."]
    pub fn set_aliases(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aliases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver_opts`.\nAn array of driver options for the network, e.g. `opts1=value`"]
    pub fn set_driver_opts(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `id`.\nThe id of the docker network to use. Please use `docker_network.id`. Using the name attribute of the docker network will lead to constant replacements."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nDeprecated attribute. The name/id of the docker network. Conflicts with `id` attribute."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_NetworksAdvanced {
    type O = terrars::BlockAssignable<Service_TaskSpec_NetworksAdvanced>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_NetworksAdvanced {}

impl BuildService_TaskSpec_NetworksAdvanced {
    pub fn build(self) -> Service_TaskSpec_NetworksAdvanced {
        Service_TaskSpec_NetworksAdvanced {
            aliases: core::default::Default::default(),
            driver_opts: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_NetworksAdvancedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_NetworksAdvancedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_NetworksAdvancedRef {
        Service_TaskSpec_NetworksAdvancedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_NetworksAdvancedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aliases` after provisioning.\nThe network aliases of the container in the specific network."]
    pub fn aliases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aliases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `driver_opts` after provisioning.\nAn array of driver options for the network, e.g. `opts1=value`"]
    pub fn driver_opts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.driver_opts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe id of the docker network to use. Please use `docker_network.id`. Using the name attribute of the docker network will lead to constant replacements."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nDeprecated attribute. The name/id of the docker network. Conflicts with `id` attribute."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_Placement_Platforms {
    architecture: terrars::PrimField<std::string::String>,
    os: terrars::PrimField<std::string::String>,
}

impl Service_TaskSpec_Placement_Platforms { }

impl terrars::ToListMappable for Service_TaskSpec_Placement_Platforms {
    type O = terrars::BlockAssignable<Service_TaskSpec_Placement_Platforms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_Placement_Platforms {
    #[doc = "The architecture, e.g. `amd64`"]
    pub architecture: terrars::PrimField<std::string::String>,
    #[doc = "The operation system, e.g. `linux`"]
    pub os: terrars::PrimField<std::string::String>,
}

impl BuildService_TaskSpec_Placement_Platforms {
    pub fn build(self) -> Service_TaskSpec_Placement_Platforms {
        Service_TaskSpec_Placement_Platforms {
            architecture: self.architecture,
            os: self.os,
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_Placement_PlatformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_Placement_PlatformsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_Placement_PlatformsRef {
        Service_TaskSpec_Placement_PlatformsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_Placement_PlatformsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\nThe architecture, e.g. `amd64`"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `os` after provisioning.\nThe operation system, e.g. `linux`"]
    pub fn os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_TaskSpec_PlacementDynamic {
    platforms: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_Placement_Platforms>>,
}

#[derive(Serialize)]
pub struct Service_TaskSpec_Placement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replicas: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platforms: std::option::Option<std::vec::Vec<Service_TaskSpec_Placement_Platforms>>,
    dynamic: Service_TaskSpec_PlacementDynamic,
}

impl Service_TaskSpec_Placement {
    #[doc = "Set the field `constraints`.\nAn array of constraints. e.g.: `node.role==manager`"]
    pub fn set_constraints(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_replicas`.\nMaximum number of replicas for per node (default value is `0`, which is unlimited)"]
    pub fn set_max_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_replicas = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefs`.\nPreferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence, e.g.: `spread=node.role.manager`"]
    pub fn set_prefs(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prefs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platforms`.\n"]
    pub fn set_platforms(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_Placement_Platforms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.platforms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.platforms = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_Placement {
    type O = terrars::BlockAssignable<Service_TaskSpec_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_Placement {}

impl BuildService_TaskSpec_Placement {
    pub fn build(self) -> Service_TaskSpec_Placement {
        Service_TaskSpec_Placement {
            constraints: core::default::Default::default(),
            max_replicas: core::default::Default::default(),
            prefs: core::default::Default::default(),
            platforms: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_PlacementRef {
        Service_TaskSpec_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_PlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `constraints` after provisioning.\nAn array of constraints. e.g.: `node.role==manager`"]
    pub fn constraints(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_replicas` after provisioning.\nMaximum number of replicas for per node (default value is `0`, which is unlimited)"]
    pub fn max_replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replicas", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefs` after provisioning.\nPreferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence, e.g.: `spread=node.role.manager`"]
    pub fn prefs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.prefs", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_Resources_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nano_cpus: std::option::Option<terrars::PrimField<f64>>,
}

impl Service_TaskSpec_Resources_Limits {
    #[doc = "Set the field `memory_bytes`.\nThe amount of memory in bytes the container allocates"]
    pub fn set_memory_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nano_cpus`.\nCPU shares in units of `1/1e9` (or `10^-9`) of the CPU. Should be at least `1000000`"]
    pub fn set_nano_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nano_cpus = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_Resources_Limits {
    type O = terrars::BlockAssignable<Service_TaskSpec_Resources_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_Resources_Limits {}

impl BuildService_TaskSpec_Resources_Limits {
    pub fn build(self) -> Service_TaskSpec_Resources_Limits {
        Service_TaskSpec_Resources_Limits {
            memory_bytes: core::default::Default::default(),
            nano_cpus: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_Resources_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_Resources_LimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_Resources_LimitsRef {
        Service_TaskSpec_Resources_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_Resources_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `memory_bytes` after provisioning.\nThe amount of memory in bytes the container allocates"]
    pub fn memory_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nano_cpus` after provisioning.\nCPU shares in units of `1/1e9` (or `10^-9`) of the CPU. Should be at least `1000000`"]
    pub fn nano_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nano_cpus", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_Resources_Reservation_GenericResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discrete_resources_spec: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_resources_spec: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl Service_TaskSpec_Resources_Reservation_GenericResources {
    #[doc = "Set the field `discrete_resources_spec`.\nThe Integer resources"]
    pub fn set_discrete_resources_spec(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.discrete_resources_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `named_resources_spec`.\nThe String resources"]
    pub fn set_named_resources_spec(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.named_resources_spec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_Resources_Reservation_GenericResources {
    type O = terrars::BlockAssignable<Service_TaskSpec_Resources_Reservation_GenericResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_Resources_Reservation_GenericResources {}

impl BuildService_TaskSpec_Resources_Reservation_GenericResources {
    pub fn build(self) -> Service_TaskSpec_Resources_Reservation_GenericResources {
        Service_TaskSpec_Resources_Reservation_GenericResources {
            discrete_resources_spec: core::default::Default::default(),
            named_resources_spec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_Resources_Reservation_GenericResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_Resources_Reservation_GenericResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Service_TaskSpec_Resources_Reservation_GenericResourcesRef {
        Service_TaskSpec_Resources_Reservation_GenericResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_Resources_Reservation_GenericResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `discrete_resources_spec` after provisioning.\nThe Integer resources"]
    pub fn discrete_resources_spec(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.discrete_resources_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `named_resources_spec` after provisioning.\nThe String resources"]
    pub fn named_resources_spec(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.named_resources_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_TaskSpec_Resources_ReservationDynamic {
    generic_resources: std
    ::option
    ::Option<terrars::DynamicBlock<Service_TaskSpec_Resources_Reservation_GenericResources>>,
}

#[derive(Serialize)]
pub struct Service_TaskSpec_Resources_Reservation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nano_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generic_resources: std::option::Option<std::vec::Vec<Service_TaskSpec_Resources_Reservation_GenericResources>>,
    dynamic: Service_TaskSpec_Resources_ReservationDynamic,
}

impl Service_TaskSpec_Resources_Reservation {
    #[doc = "Set the field `memory_bytes`.\nThe amount of memory in bytes the container allocates"]
    pub fn set_memory_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nano_cpus`.\nCPU shares in units of 1/1e9 (or 10^-9) of the CPU. Should be at least `1000000`"]
    pub fn set_nano_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nano_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generic_resources`.\n"]
    pub fn set_generic_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Service_TaskSpec_Resources_Reservation_GenericResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generic_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generic_resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_Resources_Reservation {
    type O = terrars::BlockAssignable<Service_TaskSpec_Resources_Reservation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_Resources_Reservation {}

impl BuildService_TaskSpec_Resources_Reservation {
    pub fn build(self) -> Service_TaskSpec_Resources_Reservation {
        Service_TaskSpec_Resources_Reservation {
            memory_bytes: core::default::Default::default(),
            nano_cpus: core::default::Default::default(),
            generic_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_Resources_ReservationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_Resources_ReservationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_Resources_ReservationRef {
        Service_TaskSpec_Resources_ReservationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_Resources_ReservationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `memory_bytes` after provisioning.\nThe amount of memory in bytes the container allocates"]
    pub fn memory_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nano_cpus` after provisioning.\nCPU shares in units of 1/1e9 (or 10^-9) of the CPU. Should be at least `1000000`"]
    pub fn nano_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nano_cpus", self.base))
    }

    #[doc = "Get a reference to the value of field `generic_resources` after provisioning.\n"]
    pub fn generic_resources(&self) -> terrars::ListRef<Service_TaskSpec_Resources_Reservation_GenericResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_TaskSpec_ResourcesDynamic {
    limits: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_Resources_Limits>>,
    reservation: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_Resources_Reservation>>,
}

#[derive(Serialize)]
pub struct Service_TaskSpec_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<Service_TaskSpec_Resources_Limits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation: std::option::Option<std::vec::Vec<Service_TaskSpec_Resources_Reservation>>,
    dynamic: Service_TaskSpec_ResourcesDynamic,
}

impl Service_TaskSpec_Resources {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_Resources_Limits>>,
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

    #[doc = "Set the field `reservation`.\n"]
    pub fn set_reservation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_Resources_Reservation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reservation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_Resources {
    type O = terrars::BlockAssignable<Service_TaskSpec_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_Resources {}

impl BuildService_TaskSpec_Resources {
    pub fn build(self) -> Service_TaskSpec_Resources {
        Service_TaskSpec_Resources {
            limits: core::default::Default::default(),
            reservation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_ResourcesRef {
        Service_TaskSpec_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<Service_TaskSpec_Resources_LimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation` after provisioning.\n"]
    pub fn reservation(&self) -> terrars::ListRef<Service_TaskSpec_Resources_ReservationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_TaskSpec_RestartPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Service_TaskSpec_RestartPolicy {
    #[doc = "Set the field `condition`.\nCondition for restart"]
    pub fn set_condition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delay`.\nDelay between restart attempts (ms|s|m|h)"]
    pub fn set_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delay = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_attempts`.\nMaximum attempts to restart a given container before giving up (default value is `0`, which is ignored)"]
    pub fn set_max_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `window`.\nThe time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)"]
    pub fn set_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec_RestartPolicy {
    type O = terrars::BlockAssignable<Service_TaskSpec_RestartPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec_RestartPolicy {}

impl BuildService_TaskSpec_RestartPolicy {
    pub fn build(self) -> Service_TaskSpec_RestartPolicy {
        Service_TaskSpec_RestartPolicy {
            condition: core::default::Default::default(),
            delay: core::default::Default::default(),
            max_attempts: core::default::Default::default(),
            window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpec_RestartPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpec_RestartPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpec_RestartPolicyRef {
        Service_TaskSpec_RestartPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpec_RestartPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\nCondition for restart"]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delay` after provisioning.\nDelay between restart attempts (ms|s|m|h)"]
    pub fn delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_attempts` after provisioning.\nMaximum attempts to restart a given container before giving up (default value is `0`, which is ignored)"]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `window` after provisioning.\nThe time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)"]
    pub fn window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize, Default)]
struct Service_TaskSpecDynamic {
    container_spec: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_ContainerSpec>>,
    log_driver: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_LogDriver>>,
    networks_advanced: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_NetworksAdvanced>>,
    placement: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_Placement>>,
    resources: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_Resources>>,
    restart_policy: std::option::Option<terrars::DynamicBlock<Service_TaskSpec_RestartPolicy>>,
}

#[derive(Serialize)]
pub struct Service_TaskSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_spec: std::option::Option<std::vec::Vec<Service_TaskSpec_ContainerSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_driver: std::option::Option<std::vec::Vec<Service_TaskSpec_LogDriver>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks_advanced: std::option::Option<std::vec::Vec<Service_TaskSpec_NetworksAdvanced>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<std::vec::Vec<Service_TaskSpec_Placement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<Service_TaskSpec_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_policy: std::option::Option<std::vec::Vec<Service_TaskSpec_RestartPolicy>>,
    dynamic: Service_TaskSpecDynamic,
}

impl Service_TaskSpec {
    #[doc =
        "Set the field `force_update`.\nA counter that triggers an update even if no relevant parameters have been changed. See the [spec](https://github.com/docker/swarmkit/blob/master/api/specs.proto#L126)."]
    pub fn set_force_update(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.force_update = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `runtime`.\nRuntime is the type of runtime specified for the task executor. See the [types](https://github.com/moby/moby/blob/master/api/types/swarm/runtime.go)."]
    pub fn set_runtime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.runtime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_spec`.\n"]
    pub fn set_container_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_ContainerSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_driver`.\n"]
    pub fn set_log_driver(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_LogDriver>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_driver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_driver = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `networks_advanced`.\n"]
    pub fn set_networks_advanced(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_NetworksAdvanced>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.networks_advanced = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.networks_advanced = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement`.\n"]
    pub fn set_placement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_Placement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_Resources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restart_policy`.\n"]
    pub fn set_restart_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Service_TaskSpec_RestartPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.restart_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.restart_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Service_TaskSpec {
    type O = terrars::BlockAssignable<Service_TaskSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_TaskSpec {}

impl BuildService_TaskSpec {
    pub fn build(self) -> Service_TaskSpec {
        Service_TaskSpec {
            force_update: core::default::Default::default(),
            runtime: core::default::Default::default(),
            container_spec: core::default::Default::default(),
            log_driver: core::default::Default::default(),
            networks_advanced: core::default::Default::default(),
            placement: core::default::Default::default(),
            resources: core::default::Default::default(),
            restart_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_TaskSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_TaskSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_TaskSpecRef {
        Service_TaskSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_TaskSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `force_update` after provisioning.\nA counter that triggers an update even if no relevant parameters have been changed. See the [spec](https://github.com/docker/swarmkit/blob/master/api/specs.proto#L126)."]
    pub fn force_update(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update", self.base))
    }

    #[doc =
        "Get a reference to the value of field `runtime` after provisioning.\nRuntime is the type of runtime specified for the task executor. See the [types](https://github.com/moby/moby/blob/master/api/types/swarm/runtime.go)."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `container_spec` after provisioning.\n"]
    pub fn container_spec(&self) -> terrars::ListRef<Service_TaskSpec_ContainerSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `log_driver` after provisioning.\n"]
    pub fn log_driver(&self) -> terrars::ListRef<Service_TaskSpec_LogDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_driver", self.base))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<Service_TaskSpec_PlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<Service_TaskSpec_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_policy` after provisioning.\n"]
    pub fn restart_policy(&self) -> terrars::ListRef<Service_TaskSpec_RestartPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restart_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct Service_UpdateConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_failure_ratio: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism: std::option::Option<terrars::PrimField<f64>>,
}

impl Service_UpdateConfig {
    #[doc = "Set the field `delay`.\nDelay between task updates `(ns|us|ms|s|m|h)`. Defaults to `0s`."]
    pub fn set_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delay = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `failure_action`.\nAction on update failure: `pause`, `continue` or `rollback`. Defaults to `pause`."]
    pub fn set_failure_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failure_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_failure_ratio`.\nFailure rate to tolerate during an update. Defaults to `0.0`."]
    pub fn set_max_failure_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_failure_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `monitor`.\nDuration after each task update to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`."]
    pub fn set_monitor(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.monitor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\nUpdate order: either 'stop-first' or 'start-first'. Defaults to `stop-first`."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelism`.\nMaximum number of tasks to be updated in one iteration. Defaults to `1`"]
    pub fn set_parallelism(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallelism = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Service_UpdateConfig {
    type O = terrars::BlockAssignable<Service_UpdateConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildService_UpdateConfig {}

impl BuildService_UpdateConfig {
    pub fn build(self) -> Service_UpdateConfig {
        Service_UpdateConfig {
            delay: core::default::Default::default(),
            failure_action: core::default::Default::default(),
            max_failure_ratio: core::default::Default::default(),
            monitor: core::default::Default::default(),
            order: core::default::Default::default(),
            parallelism: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Service_UpdateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Service_UpdateConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Service_UpdateConfigRef {
        Service_UpdateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Service_UpdateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delay` after provisioning.\nDelay between task updates `(ns|us|ms|s|m|h)`. Defaults to `0s`."]
    pub fn delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc =
        "Get a reference to the value of field `failure_action` after provisioning.\nAction on update failure: `pause`, `continue` or `rollback`. Defaults to `pause`."]
    pub fn failure_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_failure_ratio` after provisioning.\nFailure rate to tolerate during an update. Defaults to `0.0`."]
    pub fn max_failure_ratio(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_failure_ratio", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitor` after provisioning.\nDuration after each task update to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`."]
    pub fn monitor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nUpdate order: either 'stop-first' or 'start-first'. Defaults to `stop-first`."]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parallelism` after provisioning.\nMaximum number of tasks to be updated in one iteration. Defaults to `1`"]
    pub fn parallelism(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }
}
