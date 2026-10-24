use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct ServiceData {
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
    auth: std::option::Option<std::vec::Vec<ServiceAuthEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    converge_config: std::option::Option<std::vec::Vec<ServiceConvergeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_spec: std::option::Option<std::vec::Vec<ServiceEndpointSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<ServiceLabelsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<std::vec::Vec<ServiceModeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_config: std::option::Option<std::vec::Vec<ServiceRollbackConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_spec: std::option::Option<std::vec::Vec<ServiceTaskSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_config: std::option::Option<std::vec::Vec<ServiceUpdateConfigEl>>,
    dynamic: ServiceDynamic,
}

struct Service_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ServiceData>,
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
    pub fn set_auth(self, v: impl std::convert::Into<terrars::BlockAssignable<ServiceAuthEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceConvergeConfigEl>>,
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
    pub fn set_endpoint_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<ServiceEndpointSpecEl>>) -> Self {
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
    pub fn set_labels(self, v: impl std::convert::Into<terrars::BlockAssignable<ServiceLabelsEl>>) -> Self {
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
    pub fn set_mode(self, v: impl std::convert::Into<terrars::BlockAssignable<ServiceModeEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceRollbackConfigEl>>,
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
    pub fn set_task_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecEl>>) -> Self {
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
    pub fn set_update_config(self, v: impl std::convert::Into<terrars::BlockAssignable<ServiceUpdateConfigEl>>) -> Self {
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
    pub fn auth(&self) -> terrars::ListRef<ServiceAuthElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `converge_config` after provisioning.\n"]
    pub fn converge_config(&self) -> terrars::ListRef<ServiceConvergeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.converge_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_spec` after provisioning.\n"]
    pub fn endpoint_spec(&self) -> terrars::ListRef<ServiceEndpointSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::ListRef<ServiceModeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollback_config` after provisioning.\n"]
    pub fn rollback_config(&self) -> terrars::ListRef<ServiceRollbackConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_spec` after provisioning.\n"]
    pub fn task_spec(&self) -> terrars::ListRef<ServiceTaskSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<ServiceUpdateConfigElRef> {
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
            data: std::cell::RefCell::new(ServiceData {
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
    pub fn auth(&self) -> terrars::ListRef<ServiceAuthElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `converge_config` after provisioning.\n"]
    pub fn converge_config(&self) -> terrars::ListRef<ServiceConvergeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.converge_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_spec` after provisioning.\n"]
    pub fn endpoint_spec(&self) -> terrars::ListRef<ServiceEndpointSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::ListRef<ServiceModeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollback_config` after provisioning.\n"]
    pub fn rollback_config(&self) -> terrars::ListRef<ServiceRollbackConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_spec` after provisioning.\n"]
    pub fn task_spec(&self) -> terrars::ListRef<ServiceTaskSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> terrars::ListRef<ServiceUpdateConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ServiceAuthEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    server_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServiceAuthEl {
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

impl terrars::ToListMappable for ServiceAuthEl {
    type O = terrars::BlockAssignable<ServiceAuthEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceAuthEl {
    #[doc = "The address of the server for the authentication"]
    pub server_address: terrars::PrimField<std::string::String>,
}

impl BuildServiceAuthEl {
    pub fn build(self) -> ServiceAuthEl {
        ServiceAuthEl {
            password: core::default::Default::default(),
            server_address: self.server_address,
            username: core::default::Default::default(),
        }
    }
}

pub struct ServiceAuthElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceAuthElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceAuthElRef {
        ServiceAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceAuthElRef {
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
pub struct ServiceConvergeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServiceConvergeConfigEl {
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

impl terrars::ToListMappable for ServiceConvergeConfigEl {
    type O = terrars::BlockAssignable<ServiceConvergeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceConvergeConfigEl {}

impl BuildServiceConvergeConfigEl {
    pub fn build(self) -> ServiceConvergeConfigEl {
        ServiceConvergeConfigEl {
            delay: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

pub struct ServiceConvergeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceConvergeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceConvergeConfigElRef {
        ServiceConvergeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceConvergeConfigElRef {
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
pub struct ServiceEndpointSpecElPortsEl {
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

impl ServiceEndpointSpecElPortsEl {
    #[doc = "Set the field `name`.\nA random name for the port"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\nRrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`."]
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

impl terrars::ToListMappable for ServiceEndpointSpecElPortsEl {
    type O = terrars::BlockAssignable<ServiceEndpointSpecElPortsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceEndpointSpecElPortsEl {
    #[doc = "The port inside the container"]
    pub target_port: terrars::PrimField<f64>,
}

impl BuildServiceEndpointSpecElPortsEl {
    pub fn build(self) -> ServiceEndpointSpecElPortsEl {
        ServiceEndpointSpecElPortsEl {
            name: core::default::Default::default(),
            protocol: core::default::Default::default(),
            publish_mode: core::default::Default::default(),
            published_port: core::default::Default::default(),
            target_port: self.target_port,
        }
    }
}

pub struct ServiceEndpointSpecElPortsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceEndpointSpecElPortsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceEndpointSpecElPortsElRef {
        ServiceEndpointSpecElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceEndpointSpecElPortsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA random name for the port"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nRrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`."]
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
struct ServiceEndpointSpecElDynamic {
    ports: std::option::Option<terrars::DynamicBlock<ServiceEndpointSpecElPortsEl>>,
}

#[derive(Serialize)]
pub struct ServiceEndpointSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<ServiceEndpointSpecElPortsEl>>,
    dynamic: ServiceEndpointSpecElDynamic,
}

impl ServiceEndpointSpecEl {
    #[doc = "Set the field `mode`.\nThe mode of resolution to use for internal load balancing between tasks"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceEndpointSpecElPortsEl>>,
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

impl terrars::ToListMappable for ServiceEndpointSpecEl {
    type O = terrars::BlockAssignable<ServiceEndpointSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceEndpointSpecEl {}

impl BuildServiceEndpointSpecEl {
    pub fn build(self) -> ServiceEndpointSpecEl {
        ServiceEndpointSpecEl {
            mode: core::default::Default::default(),
            ports: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ServiceEndpointSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceEndpointSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceEndpointSpecElRef {
        ServiceEndpointSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceEndpointSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode of resolution to use for internal load balancing between tasks"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<ServiceEndpointSpecElPortsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceLabelsEl {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ServiceLabelsEl { }

impl terrars::ToListMappable for ServiceLabelsEl {
    type O = terrars::BlockAssignable<ServiceLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceLabelsEl {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildServiceLabelsEl {
    pub fn build(self) -> ServiceLabelsEl {
        ServiceLabelsEl {
            label: self.label,
            value: self.value,
        }
    }
}

pub struct ServiceLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceLabelsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceLabelsElRef {
        ServiceLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceLabelsElRef {
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
pub struct ServiceModeElReplicatedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::PrimField<f64>>,
}

impl ServiceModeElReplicatedEl {
    #[doc = "Set the field `replicas`.\nThe amount of replicas of the service. Defaults to `1`"]
    pub fn set_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServiceModeElReplicatedEl {
    type O = terrars::BlockAssignable<ServiceModeElReplicatedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceModeElReplicatedEl {}

impl BuildServiceModeElReplicatedEl {
    pub fn build(self) -> ServiceModeElReplicatedEl {
        ServiceModeElReplicatedEl { replicas: core::default::Default::default() }
    }
}

pub struct ServiceModeElReplicatedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceModeElReplicatedElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceModeElReplicatedElRef {
        ServiceModeElReplicatedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceModeElReplicatedElRef {
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
struct ServiceModeElDynamic {
    replicated: std::option::Option<terrars::DynamicBlock<ServiceModeElReplicatedEl>>,
}

#[derive(Serialize)]
pub struct ServiceModeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicated: std::option::Option<std::vec::Vec<ServiceModeElReplicatedEl>>,
    dynamic: ServiceModeElDynamic,
}

impl ServiceModeEl {
    #[doc = "Set the field `global`.\nThe global service mode. Defaults to `false`"]
    pub fn set_global(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.global = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replicated`.\n"]
    pub fn set_replicated(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceModeElReplicatedEl>>,
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

impl terrars::ToListMappable for ServiceModeEl {
    type O = terrars::BlockAssignable<ServiceModeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceModeEl {}

impl BuildServiceModeEl {
    pub fn build(self) -> ServiceModeEl {
        ServiceModeEl {
            global: core::default::Default::default(),
            replicated: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ServiceModeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceModeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceModeElRef {
        ServiceModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceModeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `global` after provisioning.\nThe global service mode. Defaults to `false`"]
    pub fn global(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global", self.base))
    }

    #[doc = "Get a reference to the value of field `replicated` after provisioning.\n"]
    pub fn replicated(&self) -> terrars::ListRef<ServiceModeElReplicatedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicated", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceRollbackConfigEl {
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

impl ServiceRollbackConfigEl {
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

impl terrars::ToListMappable for ServiceRollbackConfigEl {
    type O = terrars::BlockAssignable<ServiceRollbackConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceRollbackConfigEl {}

impl BuildServiceRollbackConfigEl {
    pub fn build(self) -> ServiceRollbackConfigEl {
        ServiceRollbackConfigEl {
            delay: core::default::Default::default(),
            failure_action: core::default::Default::default(),
            max_failure_ratio: core::default::Default::default(),
            monitor: core::default::Default::default(),
            order: core::default::Default::default(),
            parallelism: core::default::Default::default(),
        }
    }
}

pub struct ServiceRollbackConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceRollbackConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceRollbackConfigElRef {
        ServiceRollbackConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceRollbackConfigElRef {
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
pub struct ServiceTaskSpecElContainerSpecElConfigsEl {
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

impl ServiceTaskSpecElContainerSpecElConfigsEl {
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElConfigsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElConfigsEl {
    #[doc = "ID of the specific config that we're referencing"]
    pub config_id: terrars::PrimField<std::string::String>,
    #[doc = "Represents the final filename in the filesystem"]
    pub file_name: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElContainerSpecElConfigsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElConfigsEl {
        ServiceTaskSpecElContainerSpecElConfigsEl {
            config_id: self.config_id,
            config_name: core::default::Default::default(),
            file_gid: core::default::Default::default(),
            file_mode: core::default::Default::default(),
            file_name: self.file_name,
            file_uid: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElConfigsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElContainerSpecElConfigsElRef {
        ServiceTaskSpecElContainerSpecElConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElConfigsElRef {
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
pub struct ServiceTaskSpecElContainerSpecElDnsConfigEl {
    nameservers: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ServiceTaskSpecElContainerSpecElDnsConfigEl {
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElDnsConfigEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElDnsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElDnsConfigEl {
    #[doc = "The IP addresses of the name servers"]
    pub nameservers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildServiceTaskSpecElContainerSpecElDnsConfigEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElDnsConfigEl {
        ServiceTaskSpecElContainerSpecElDnsConfigEl {
            nameservers: self.nameservers,
            options: core::default::Default::default(),
            search: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElDnsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElDnsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElDnsConfigElRef {
        ServiceTaskSpecElContainerSpecElDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElDnsConfigElRef {
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
pub struct ServiceTaskSpecElContainerSpecElHealthcheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_period: std::option::Option<terrars::PrimField<std::string::String>>,
    test: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServiceTaskSpecElContainerSpecElHealthcheckEl {
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

    #[doc = "Set the field `timeout`.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElHealthcheckEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElHealthcheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElHealthcheckEl {
    #[doc = "The test to perform as list"]
    pub test: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildServiceTaskSpecElContainerSpecElHealthcheckEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElHealthcheckEl {
        ServiceTaskSpecElContainerSpecElHealthcheckEl {
            interval: core::default::Default::default(),
            retries: core::default::Default::default(),
            start_period: core::default::Default::default(),
            test: self.test,
            timeout: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElHealthcheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElHealthcheckElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElHealthcheckElRef {
        ServiceTaskSpecElContainerSpecElHealthcheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElHealthcheckElRef {
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
pub struct ServiceTaskSpecElContainerSpecElHostsEl {
    host: terrars::PrimField<std::string::String>,
    ip: terrars::PrimField<std::string::String>,
}

impl ServiceTaskSpecElContainerSpecElHostsEl { }

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElHostsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElHostsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElHostsEl {
    #[doc = "The name of the host"]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "The ip of the host"]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElContainerSpecElHostsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElHostsEl {
        ServiceTaskSpecElContainerSpecElHostsEl {
            host: self.host,
            ip: self.ip,
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElHostsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElHostsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElContainerSpecElHostsElRef {
        ServiceTaskSpecElContainerSpecElHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElHostsElRef {
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
pub struct ServiceTaskSpecElContainerSpecElLabelsEl {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ServiceTaskSpecElContainerSpecElLabelsEl { }

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElLabelsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElLabelsEl {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElContainerSpecElLabelsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElLabelsEl {
        ServiceTaskSpecElContainerSpecElLabelsEl {
            label: self.label,
            value: self.value,
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElLabelsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElContainerSpecElLabelsElRef {
        ServiceTaskSpecElContainerSpecElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElLabelsElRef {
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
pub struct ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {
    #[doc =
        "Set the field `propagation`.\nBind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`"]
    pub fn set_propagation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.propagation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {}

impl BuildServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {
        ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl { propagation: core::default::Default::default() }
    }
}

pub struct ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
        ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
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
pub struct ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_bytes: std::option::Option<terrars::PrimField<f64>>,
}

impl ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {}

impl BuildServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
        ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
            mode: core::default::Default::default(),
            size_bytes: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
        ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
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

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl { }

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
        ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
            label: self.label,
            value: self.value,
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
        ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
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

#[derive(Serialize, Default)]
struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElDynamic {
    labels: std
    ::option
    ::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_copy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl>>,
    dynamic: ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElDynamic,
}

impl ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
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
    pub fn set_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl>>,
    ) -> Self {
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {}

impl BuildServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
        ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
            driver_name: core::default::Default::default(),
            driver_options: core::default::Default::default(),
            no_copy: core::default::Default::default(),
            labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
        ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
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
struct ServiceTaskSpecElContainerSpecElMountsElDynamic {
    bind_options: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl>>,
    tmpfs_options: std
    ::option
    ::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl>>,
    volume_options: std
    ::option
    ::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElMountsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bind_options: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tmpfs_options: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_options: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl>>,
    dynamic: ServiceTaskSpecElContainerSpecElMountsElDynamic,
}

impl ServiceTaskSpecElContainerSpecElMountsEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl>>,
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElMountsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElMountsEl {
    #[doc = "Container path"]
    pub target: terrars::PrimField<std::string::String>,
    #[doc = "The mount type"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElContainerSpecElMountsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElMountsEl {
        ServiceTaskSpecElContainerSpecElMountsEl {
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

pub struct ServiceTaskSpecElContainerSpecElMountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElMountsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElContainerSpecElMountsElRef {
        ServiceTaskSpecElContainerSpecElMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElRef {
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
    pub fn bind_options(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bind_options", self.base))
    }

    #[doc = "Get a reference to the value of field `tmpfs_options` after provisioning.\n"]
    pub fn tmpfs_options(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tmpfs_options", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_options` after provisioning.\n"]
    pub fn volume_options(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {}

impl BuildServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
        ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
            file: core::default::Default::default(),
            registry: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
        ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
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
pub struct ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
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

impl ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {}

impl BuildServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
        ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
            disable: core::default::Default::default(),
            level: core::default::Default::default(),
            role: core::default::Default::default(),
            type_: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
        ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
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
struct ServiceTaskSpecElContainerSpecElPrivilegesElDynamic {
    credential_spec: std
    ::option
    ::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl>>,
    se_linux_context: std
    ::option
    ::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElPrivilegesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_spec: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    se_linux_context: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl>>,
    dynamic: ServiceTaskSpecElContainerSpecElPrivilegesElDynamic,
}

impl ServiceTaskSpecElContainerSpecElPrivilegesEl {
    #[doc = "Set the field `credential_spec`.\n"]
    pub fn set_credential_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl>>,
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
                    ::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl>>,
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElPrivilegesEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElPrivilegesEl {}

impl BuildServiceTaskSpecElContainerSpecElPrivilegesEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElPrivilegesEl {
        ServiceTaskSpecElContainerSpecElPrivilegesEl {
            credential_spec: core::default::Default::default(),
            se_linux_context: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElPrivilegesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElPrivilegesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElContainerSpecElPrivilegesElRef {
        ServiceTaskSpecElContainerSpecElPrivilegesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credential_spec` after provisioning.\n"]
    pub fn credential_spec(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credential_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `se_linux_context` after provisioning.\n"]
    pub fn se_linux_context(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.se_linux_context", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElSecretsEl {
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

impl ServiceTaskSpecElContainerSpecElSecretsEl {
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecElSecretsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElSecretsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElSecretsEl {
    #[doc = "Represents the final filename in the filesystem"]
    pub file_name: terrars::PrimField<std::string::String>,
    #[doc = "ID of the specific secret that we're referencing"]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElContainerSpecElSecretsEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecElSecretsEl {
        ServiceTaskSpecElContainerSpecElSecretsEl {
            file_gid: core::default::Default::default(),
            file_mode: core::default::Default::default(),
            file_name: self.file_name,
            file_uid: core::default::Default::default(),
            secret_id: self.secret_id,
            secret_name: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElSecretsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElSecretsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElContainerSpecElSecretsElRef {
        ServiceTaskSpecElContainerSpecElSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElSecretsElRef {
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
struct ServiceTaskSpecElContainerSpecElDynamic {
    configs: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElConfigsEl>>,
    dns_config: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElDnsConfigEl>>,
    healthcheck: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElHealthcheckEl>>,
    hosts: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElHostsEl>>,
    labels: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElLabelsEl>>,
    mounts: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElMountsEl>>,
    privileges: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElPrivilegesEl>>,
    secrets: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecElSecretsEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
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
    configs: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElConfigsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_config: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElDnsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthcheck: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElHealthcheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElHostsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElLabelsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mounts: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElMountsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileges: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElPrivilegesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecElSecretsEl>>,
    dynamic: ServiceTaskSpecElContainerSpecElDynamic,
}

impl ServiceTaskSpecElContainerSpecEl {
    #[doc = "Set the field `args`.\nArguments to the command"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElConfigsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElDnsConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElHealthcheckEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElHostsEl>>,
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
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElLabelsEl>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElMountsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecElSecretsEl>>,
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

impl terrars::ToListMappable for ServiceTaskSpecElContainerSpecEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElContainerSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecEl {
    #[doc =
        "The image name to use for the containers of the service, like `nginx:1.17.6`. Also use the data-source or resource of `docker_image` with the `repo_digest` or `docker_registry_image` with the `name` attribute for this, as shown in the examples."]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElContainerSpecEl {
    pub fn build(self) -> ServiceTaskSpecElContainerSpecEl {
        ServiceTaskSpecElContainerSpecEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            dir: core::default::Default::default(),
            env: core::default::Default::default(),
            groups: core::default::Default::default(),
            hostname: core::default::Default::default(),
            image: self.image,
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

pub struct ServiceTaskSpecElContainerSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElContainerSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElContainerSpecElRef {
        ServiceTaskSpecElContainerSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\nArguments to the command"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
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
    pub fn dns_config(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElDnsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_config", self.base))
    }

    #[doc = "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElHealthcheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.base))
    }

    #[doc = "Get a reference to the value of field `privileges` after provisioning.\n"]
    pub fn privileges(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElPrivilegesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.privileges", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElLogDriverEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ServiceTaskSpecElLogDriverEl {
    #[doc = "Set the field `options`.\nThe options for the logging driver"]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServiceTaskSpecElLogDriverEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElLogDriverEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElLogDriverEl {
    #[doc = "The logging driver to use"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElLogDriverEl {
    pub fn build(self) -> ServiceTaskSpecElLogDriverEl {
        ServiceTaskSpecElLogDriverEl {
            name: self.name,
            options: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElLogDriverElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElLogDriverElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElLogDriverElRef {
        ServiceTaskSpecElLogDriverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElLogDriverElRef {
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
pub struct ServiceTaskSpecElNetworksAdvancedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aliases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_opts: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
}

impl ServiceTaskSpecElNetworksAdvancedEl {
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
}

impl terrars::ToListMappable for ServiceTaskSpecElNetworksAdvancedEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElNetworksAdvancedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElNetworksAdvancedEl {
    #[doc = "The name/id of the network."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElNetworksAdvancedEl {
    pub fn build(self) -> ServiceTaskSpecElNetworksAdvancedEl {
        ServiceTaskSpecElNetworksAdvancedEl {
            aliases: core::default::Default::default(),
            driver_opts: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct ServiceTaskSpecElNetworksAdvancedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElNetworksAdvancedElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElNetworksAdvancedElRef {
        ServiceTaskSpecElNetworksAdvancedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElNetworksAdvancedElRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name/id of the network."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElPlacementElPlatformsEl {
    architecture: terrars::PrimField<std::string::String>,
    os: terrars::PrimField<std::string::String>,
}

impl ServiceTaskSpecElPlacementElPlatformsEl { }

impl terrars::ToListMappable for ServiceTaskSpecElPlacementElPlatformsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElPlacementElPlatformsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElPlacementElPlatformsEl {
    #[doc = "The architecture, e.g. `amd64`"]
    pub architecture: terrars::PrimField<std::string::String>,
    #[doc = "The operation system, e.g. `linux`"]
    pub os: terrars::PrimField<std::string::String>,
}

impl BuildServiceTaskSpecElPlacementElPlatformsEl {
    pub fn build(self) -> ServiceTaskSpecElPlacementElPlatformsEl {
        ServiceTaskSpecElPlacementElPlatformsEl {
            architecture: self.architecture,
            os: self.os,
        }
    }
}

pub struct ServiceTaskSpecElPlacementElPlatformsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElPlacementElPlatformsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElPlacementElPlatformsElRef {
        ServiceTaskSpecElPlacementElPlatformsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElPlacementElPlatformsElRef {
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
struct ServiceTaskSpecElPlacementElDynamic {
    platforms: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElPlacementElPlatformsEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElPlacementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replicas: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platforms: std::option::Option<std::vec::Vec<ServiceTaskSpecElPlacementElPlatformsEl>>,
    dynamic: ServiceTaskSpecElPlacementElDynamic,
}

impl ServiceTaskSpecElPlacementEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElPlacementElPlatformsEl>>,
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

impl terrars::ToListMappable for ServiceTaskSpecElPlacementEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElPlacementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElPlacementEl {}

impl BuildServiceTaskSpecElPlacementEl {
    pub fn build(self) -> ServiceTaskSpecElPlacementEl {
        ServiceTaskSpecElPlacementEl {
            constraints: core::default::Default::default(),
            max_replicas: core::default::Default::default(),
            prefs: core::default::Default::default(),
            platforms: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElPlacementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElPlacementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElPlacementElRef {
        ServiceTaskSpecElPlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElPlacementElRef {
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
pub struct ServiceTaskSpecElResourcesElLimitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nano_cpus: std::option::Option<terrars::PrimField<f64>>,
}

impl ServiceTaskSpecElResourcesElLimitsEl {
    #[doc = "Set the field `memory_bytes`.\nThe amounf of memory in bytes the container allocates"]
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

impl terrars::ToListMappable for ServiceTaskSpecElResourcesElLimitsEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElResourcesElLimitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElResourcesElLimitsEl {}

impl BuildServiceTaskSpecElResourcesElLimitsEl {
    pub fn build(self) -> ServiceTaskSpecElResourcesElLimitsEl {
        ServiceTaskSpecElResourcesElLimitsEl {
            memory_bytes: core::default::Default::default(),
            nano_cpus: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElResourcesElLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElResourcesElLimitsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElResourcesElLimitsElRef {
        ServiceTaskSpecElResourcesElLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElResourcesElLimitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `memory_bytes` after provisioning.\nThe amounf of memory in bytes the container allocates"]
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
pub struct ServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discrete_resources_spec: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_resources_spec: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
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

impl terrars::ToListMappable for ServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElResourcesElReservationElGenericResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElResourcesElReservationElGenericResourcesEl {}

impl BuildServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
    pub fn build(self) -> ServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
        ServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
            discrete_resources_spec: core::default::Default::default(),
            named_resources_spec: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
        ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
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
struct ServiceTaskSpecElResourcesElReservationElDynamic {
    generic_resources: std
    ::option
    ::Option<terrars::DynamicBlock<ServiceTaskSpecElResourcesElReservationElGenericResourcesEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElResourcesElReservationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nano_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generic_resources: std::option::Option<std::vec::Vec<ServiceTaskSpecElResourcesElReservationElGenericResourcesEl>>,
    dynamic: ServiceTaskSpecElResourcesElReservationElDynamic,
}

impl ServiceTaskSpecElResourcesElReservationEl {
    #[doc = "Set the field `memory_bytes`.\nThe amounf of memory in bytes the container allocates"]
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
                    ::Into<terrars::BlockAssignable<ServiceTaskSpecElResourcesElReservationElGenericResourcesEl>>,
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

impl terrars::ToListMappable for ServiceTaskSpecElResourcesElReservationEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElResourcesElReservationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElResourcesElReservationEl {}

impl BuildServiceTaskSpecElResourcesElReservationEl {
    pub fn build(self) -> ServiceTaskSpecElResourcesElReservationEl {
        ServiceTaskSpecElResourcesElReservationEl {
            memory_bytes: core::default::Default::default(),
            nano_cpus: core::default::Default::default(),
            generic_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElResourcesElReservationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElResourcesElReservationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElResourcesElReservationElRef {
        ServiceTaskSpecElResourcesElReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElResourcesElReservationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `memory_bytes` after provisioning.\nThe amounf of memory in bytes the container allocates"]
    pub fn memory_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nano_cpus` after provisioning.\nCPU shares in units of 1/1e9 (or 10^-9) of the CPU. Should be at least `1000000`"]
    pub fn nano_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nano_cpus", self.base))
    }

    #[doc = "Get a reference to the value of field `generic_resources` after provisioning.\n"]
    pub fn generic_resources(&self) -> terrars::ListRef<ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElResourcesElDynamic {
    limits: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElResourcesElLimitsEl>>,
    reservation: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElResourcesElReservationEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<ServiceTaskSpecElResourcesElLimitsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation: std::option::Option<std::vec::Vec<ServiceTaskSpecElResourcesElReservationEl>>,
    dynamic: ServiceTaskSpecElResourcesElDynamic,
}

impl ServiceTaskSpecElResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElResourcesElLimitsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElResourcesElReservationEl>>,
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

impl terrars::ToListMappable for ServiceTaskSpecElResourcesEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElResourcesEl {}

impl BuildServiceTaskSpecElResourcesEl {
    pub fn build(self) -> ServiceTaskSpecElResourcesEl {
        ServiceTaskSpecElResourcesEl {
            limits: core::default::Default::default(),
            reservation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElResourcesElRef {
        ServiceTaskSpecElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<ServiceTaskSpecElResourcesElLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation` after provisioning.\n"]
    pub fn reservation(&self) -> terrars::ListRef<ServiceTaskSpecElResourcesElReservationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElRestartPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServiceTaskSpecElRestartPolicyEl {
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

impl terrars::ToListMappable for ServiceTaskSpecElRestartPolicyEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecElRestartPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElRestartPolicyEl {}

impl BuildServiceTaskSpecElRestartPolicyEl {
    pub fn build(self) -> ServiceTaskSpecElRestartPolicyEl {
        ServiceTaskSpecElRestartPolicyEl {
            condition: core::default::Default::default(),
            delay: core::default::Default::default(),
            max_attempts: core::default::Default::default(),
            window: core::default::Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElRestartPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElRestartPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElRestartPolicyElRef {
        ServiceTaskSpecElRestartPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElRestartPolicyElRef {
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
struct ServiceTaskSpecElDynamic {
    container_spec: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElContainerSpecEl>>,
    log_driver: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElLogDriverEl>>,
    networks_advanced: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElNetworksAdvancedEl>>,
    placement: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElPlacementEl>>,
    resources: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElResourcesEl>>,
    restart_policy: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecElRestartPolicyEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_spec: std::option::Option<std::vec::Vec<ServiceTaskSpecElContainerSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_driver: std::option::Option<std::vec::Vec<ServiceTaskSpecElLogDriverEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks_advanced: std::option::Option<std::vec::Vec<ServiceTaskSpecElNetworksAdvancedEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<std::vec::Vec<ServiceTaskSpecElPlacementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<ServiceTaskSpecElResourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart_policy: std::option::Option<std::vec::Vec<ServiceTaskSpecElRestartPolicyEl>>,
    dynamic: ServiceTaskSpecElDynamic,
}

impl ServiceTaskSpecEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElContainerSpecEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElLogDriverEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElNetworksAdvancedEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElPlacementEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElResourcesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ServiceTaskSpecElRestartPolicyEl>>,
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

impl terrars::ToListMappable for ServiceTaskSpecEl {
    type O = terrars::BlockAssignable<ServiceTaskSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecEl {}

impl BuildServiceTaskSpecEl {
    pub fn build(self) -> ServiceTaskSpecEl {
        ServiceTaskSpecEl {
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

pub struct ServiceTaskSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceTaskSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceTaskSpecElRef {
        ServiceTaskSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElRef {
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
    pub fn container_spec(&self) -> terrars::ListRef<ServiceTaskSpecElContainerSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `log_driver` after provisioning.\n"]
    pub fn log_driver(&self) -> terrars::ListRef<ServiceTaskSpecElLogDriverElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_driver", self.base))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<ServiceTaskSpecElPlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<ServiceTaskSpecElResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `restart_policy` after provisioning.\n"]
    pub fn restart_policy(&self) -> terrars::ListRef<ServiceTaskSpecElRestartPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restart_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceUpdateConfigEl {
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

impl ServiceUpdateConfigEl {
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

impl terrars::ToListMappable for ServiceUpdateConfigEl {
    type O = terrars::BlockAssignable<ServiceUpdateConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceUpdateConfigEl {}

impl BuildServiceUpdateConfigEl {
    pub fn build(self) -> ServiceUpdateConfigEl {
        ServiceUpdateConfigEl {
            delay: core::default::Default::default(),
            failure_action: core::default::Default::default(),
            max_failure_ratio: core::default::Default::default(),
            monitor: core::default::Default::default(),
            order: core::default::Default::default(),
            parallelism: core::default::Default::default(),
        }
    }
}

pub struct ServiceUpdateConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServiceUpdateConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServiceUpdateConfigElRef {
        ServiceUpdateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceUpdateConfigElRef {
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

#[derive(Serialize, Default)]
struct ServiceDynamic {
    auth: std::option::Option<terrars::DynamicBlock<ServiceAuthEl>>,
    converge_config: std::option::Option<terrars::DynamicBlock<ServiceConvergeConfigEl>>,
    endpoint_spec: std::option::Option<terrars::DynamicBlock<ServiceEndpointSpecEl>>,
    labels: std::option::Option<terrars::DynamicBlock<ServiceLabelsEl>>,
    mode: std::option::Option<terrars::DynamicBlock<ServiceModeEl>>,
    rollback_config: std::option::Option<terrars::DynamicBlock<ServiceRollbackConfigEl>>,
    task_spec: std::option::Option<terrars::DynamicBlock<ServiceTaskSpecEl>>,
    update_config: std::option::Option<terrars::DynamicBlock<ServiceUpdateConfigEl>>,
}
