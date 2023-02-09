use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct ServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<Vec<ServiceAuthEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    converge_config: Option<Vec<ServiceConvergeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_spec: Option<Vec<ServiceEndpointSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<ServiceLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<Vec<ServiceModeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback_config: Option<Vec<ServiceRollbackConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_spec: Option<Vec<ServiceTaskSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_config: Option<Vec<ServiceUpdateConfigEl>>,
    dynamic: ServiceDynamic,
}

struct Service_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServiceData>,
}

#[derive(Clone)]
pub struct Service(Rc<Service_>);

impl Service {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `auth`.\n"]
    pub fn set_auth(self, v: impl Into<BlockAssignable<ServiceAuthEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `converge_config`.\n"]
    pub fn set_converge_config(self, v: impl Into<BlockAssignable<ServiceConvergeConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().converge_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.converge_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `endpoint_spec`.\n"]
    pub fn set_endpoint_spec(self, v: impl Into<BlockAssignable<ServiceEndpointSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_spec = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl Into<BlockAssignable<ServiceLabelsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().labels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.labels = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(self, v: impl Into<BlockAssignable<ServiceModeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mode = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mode = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rollback_config`.\n"]
    pub fn set_rollback_config(self, v: impl Into<BlockAssignable<ServiceRollbackConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollback_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollback_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `task_spec`.\n"]
    pub fn set_task_spec(self, v: impl Into<BlockAssignable<ServiceTaskSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().task_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.task_spec = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `update_config`.\n"]
    pub fn set_update_config(self, v: impl Into<BlockAssignable<ServiceUpdateConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the service"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> ListRef<ServiceAuthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `converge_config` after provisioning.\n"]
    pub fn converge_config(&self) -> ListRef<ServiceConvergeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.converge_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_spec` after provisioning.\n"]
    pub fn endpoint_spec(&self) -> ListRef<ServiceEndpointSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> ListRef<ServiceModeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rollback_config` after provisioning.\n"]
    pub fn rollback_config(&self) -> ListRef<ServiceRollbackConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rollback_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `task_spec` after provisioning.\n"]
    pub fn task_spec(&self) -> ListRef<ServiceTaskSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.task_spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> ListRef<ServiceUpdateConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }
}

impl Resource for Service {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Service {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Service {
    type O = ListRef<ServiceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Service_ {
    fn extract_resource_type(&self) -> String {
        "docker_service".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildService {
    pub tf_id: String,
    #[doc= "Name of the service"]
    pub name: PrimField<String>,
}

impl BuildService {
    pub fn build(self, stack: &mut Stack) -> Service {
        let out = Service(Rc::new(Service_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServiceRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the service"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> ListRef<ServiceAuthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `converge_config` after provisioning.\n"]
    pub fn converge_config(&self) -> ListRef<ServiceConvergeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.converge_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_spec` after provisioning.\n"]
    pub fn endpoint_spec(&self) -> ListRef<ServiceEndpointSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> ListRef<ServiceModeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rollback_config` after provisioning.\n"]
    pub fn rollback_config(&self) -> ListRef<ServiceRollbackConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rollback_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `task_spec` after provisioning.\n"]
    pub fn task_spec(&self) -> ListRef<ServiceTaskSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.task_spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> ListRef<ServiceUpdateConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ServiceAuthEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    server_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

impl ServiceAuthEl {
    #[doc= "Set the field `password`.\nThe password"]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\nThe username"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceAuthEl {
    type O = BlockAssignable<ServiceAuthEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceAuthEl {
    #[doc= "The address of the server for the authentication"]
    pub server_address: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceAuthElRef {
    fn new(shared: StackShared, base: String) -> ServiceAuthElRef {
        ServiceAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceAuthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nThe password"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc= "Get a reference to the value of field `server_address` after provisioning.\nThe address of the server for the authentication"]
    pub fn server_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_address", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe username"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceConvergeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
}

impl ServiceConvergeConfigEl {
    #[doc= "Set the field `delay`.\nThe interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`."]
    pub fn set_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delay = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout`.\nThe timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`"]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceConvergeConfigEl {
    type O = BlockAssignable<ServiceConvergeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceConvergeConfigElRef {
    fn new(shared: StackShared, base: String) -> ServiceConvergeConfigElRef {
        ServiceConvergeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceConvergeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delay` after provisioning.\nThe interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`."]
    pub fn delay(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nThe timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`"]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceEndpointSpecElPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publish_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published_port: Option<PrimField<f64>>,
    target_port: PrimField<f64>,
}

impl ServiceEndpointSpecElPortsEl {
    #[doc= "Set the field `name`.\nA random name for the port"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\nRrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`."]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `publish_mode`.\nRepresents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`."]
    pub fn set_publish_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.publish_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `published_port`.\nThe port on the swarm hosts"]
    pub fn set_published_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.published_port = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceEndpointSpecElPortsEl {
    type O = BlockAssignable<ServiceEndpointSpecElPortsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceEndpointSpecElPortsEl {
    #[doc= "The port inside the container"]
    pub target_port: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceEndpointSpecElPortsElRef {
    fn new(shared: StackShared, base: String) -> ServiceEndpointSpecElPortsElRef {
        ServiceEndpointSpecElPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceEndpointSpecElPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA random name for the port"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\nRrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `publish_mode` after provisioning.\nRepresents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`."]
    pub fn publish_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.publish_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `published_port` after provisioning.\nThe port on the swarm hosts"]
    pub fn published_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.published_port", self.base))
    }

    #[doc= "Get a reference to the value of field `target_port` after provisioning.\nThe port inside the container"]
    pub fn target_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceEndpointSpecElDynamic {
    ports: Option<DynamicBlock<ServiceEndpointSpecElPortsEl>>,
}

#[derive(Serialize)]
pub struct ServiceEndpointSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<Vec<ServiceEndpointSpecElPortsEl>>,
    dynamic: ServiceEndpointSpecElDynamic,
}

impl ServiceEndpointSpecEl {
    #[doc= "Set the field `mode`.\nThe mode of resolution to use for internal load balancing between tasks"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl Into<BlockAssignable<ServiceEndpointSpecElPortsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ports = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ports = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceEndpointSpecEl {
    type O = BlockAssignable<ServiceEndpointSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceEndpointSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceEndpointSpecElRef {
    fn new(shared: StackShared, base: String) -> ServiceEndpointSpecElRef {
        ServiceEndpointSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceEndpointSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe mode of resolution to use for internal load balancing between tasks"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<ServiceEndpointSpecElPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceLabelsEl {
    label: PrimField<String>,
    value: PrimField<String>,
}

impl ServiceLabelsEl { }

impl ToListMappable for ServiceLabelsEl {
    type O = BlockAssignable<ServiceLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceLabelsEl {
    #[doc= "Name of the label"]
    pub label: PrimField<String>,
    #[doc= "Value of the label"]
    pub value: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceLabelsElRef {
    fn new(shared: StackShared, base: String) -> ServiceLabelsElRef {
        ServiceLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\nName of the label"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the label"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceModeElReplicatedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<PrimField<f64>>,
}

impl ServiceModeElReplicatedEl {
    #[doc= "Set the field `replicas`.\nThe amount of replicas of the service. Defaults to `1`"]
    pub fn set_replicas(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.replicas = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceModeElReplicatedEl {
    type O = BlockAssignable<ServiceModeElReplicatedEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceModeElReplicatedElRef {
    fn new(shared: StackShared, base: String) -> ServiceModeElReplicatedElRef {
        ServiceModeElReplicatedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceModeElReplicatedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `replicas` after provisioning.\nThe amount of replicas of the service. Defaults to `1`"]
    pub fn replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceModeElDynamic {
    replicated: Option<DynamicBlock<ServiceModeElReplicatedEl>>,
}

#[derive(Serialize)]
pub struct ServiceModeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    global: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replicated: Option<Vec<ServiceModeElReplicatedEl>>,
    dynamic: ServiceModeElDynamic,
}

impl ServiceModeEl {
    #[doc= "Set the field `global`.\nThe global service mode. Defaults to `false`"]
    pub fn set_global(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.global = Some(v.into());
        self
    }

    #[doc= "Set the field `replicated`.\n"]
    pub fn set_replicated(mut self, v: impl Into<BlockAssignable<ServiceModeElReplicatedEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replicated = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replicated = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceModeEl {
    type O = BlockAssignable<ServiceModeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceModeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceModeElRef {
    fn new(shared: StackShared, base: String) -> ServiceModeElRef {
        ServiceModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `global` after provisioning.\nThe global service mode. Defaults to `false`"]
    pub fn global(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.global", self.base))
    }

    #[doc= "Get a reference to the value of field `replicated` after provisioning.\n"]
    pub fn replicated(&self) -> ListRef<ServiceModeElReplicatedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replicated", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceRollbackConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_failure_ratio: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism: Option<PrimField<f64>>,
}

impl ServiceRollbackConfigEl {
    #[doc= "Set the field `delay`.\nDelay between task rollbacks (ns|us|ms|s|m|h). Defaults to `0s`."]
    pub fn set_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delay = Some(v.into());
        self
    }

    #[doc= "Set the field `failure_action`.\nAction on rollback failure: pause | continue. Defaults to `pause`."]
    pub fn set_failure_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failure_action = Some(v.into());
        self
    }

    #[doc= "Set the field `max_failure_ratio`.\nFailure rate to tolerate during a rollback. Defaults to `0.0`."]
    pub fn set_max_failure_ratio(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_failure_ratio = Some(v.into());
        self
    }

    #[doc= "Set the field `monitor`.\nDuration after each task rollback to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`."]
    pub fn set_monitor(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.monitor = Some(v.into());
        self
    }

    #[doc= "Set the field `order`.\nRollback order: either 'stop-first' or 'start-first'. Defaults to `stop-first`."]
    pub fn set_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.order = Some(v.into());
        self
    }

    #[doc= "Set the field `parallelism`.\nMaximum number of tasks to be rollbacked in one iteration. Defaults to `1`"]
    pub fn set_parallelism(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallelism = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceRollbackConfigEl {
    type O = BlockAssignable<ServiceRollbackConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceRollbackConfigElRef {
    fn new(shared: StackShared, base: String) -> ServiceRollbackConfigElRef {
        ServiceRollbackConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceRollbackConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delay` after provisioning.\nDelay between task rollbacks (ns|us|ms|s|m|h). Defaults to `0s`."]
    pub fn delay(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc= "Get a reference to the value of field `failure_action` after provisioning.\nAction on rollback failure: pause | continue. Defaults to `pause`."]
    pub fn failure_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_action", self.base))
    }

    #[doc= "Get a reference to the value of field `max_failure_ratio` after provisioning.\nFailure rate to tolerate during a rollback. Defaults to `0.0`."]
    pub fn max_failure_ratio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_failure_ratio", self.base))
    }

    #[doc= "Get a reference to the value of field `monitor` after provisioning.\nDuration after each task rollback to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`."]
    pub fn monitor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.base))
    }

    #[doc= "Get a reference to the value of field `order` after provisioning.\nRollback order: either 'stop-first' or 'start-first'. Defaults to `stop-first`."]
    pub fn order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc= "Get a reference to the value of field `parallelism` after provisioning.\nMaximum number of tasks to be rollbacked in one iteration. Defaults to `1`"]
    pub fn parallelism(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElConfigsEl {
    config_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_gid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_mode: Option<PrimField<f64>>,
    file_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uid: Option<PrimField<String>>,
}

impl ServiceTaskSpecElContainerSpecElConfigsEl {
    #[doc= "Set the field `config_name`.\nName of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID"]
    pub fn set_config_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.config_name = Some(v.into());
        self
    }

    #[doc= "Set the field `file_gid`.\nRepresents the file GID. Defaults to `0`."]
    pub fn set_file_gid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_gid = Some(v.into());
        self
    }

    #[doc= "Set the field `file_mode`.\nRepresents represents the FileMode of the file. Defaults to `0o444`."]
    pub fn set_file_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.file_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `file_uid`.\nRepresents the file UID. Defaults to `0`."]
    pub fn set_file_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_uid = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElConfigsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElConfigsEl {
    #[doc= "ID of the specific config that we're referencing"]
    pub config_id: PrimField<String>,
    #[doc= "Represents the final filename in the filesystem"]
    pub file_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElConfigsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElConfigsElRef {
        ServiceTaskSpecElContainerSpecElConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `config_id` after provisioning.\nID of the specific config that we're referencing"]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.base))
    }

    #[doc= "Get a reference to the value of field `config_name` after provisioning.\nName of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID"]
    pub fn config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_name", self.base))
    }

    #[doc= "Get a reference to the value of field `file_gid` after provisioning.\nRepresents the file GID. Defaults to `0`."]
    pub fn file_gid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_gid", self.base))
    }

    #[doc= "Get a reference to the value of field `file_mode` after provisioning.\nRepresents represents the FileMode of the file. Defaults to `0o444`."]
    pub fn file_mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nRepresents the final filename in the filesystem"]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc= "Get a reference to the value of field `file_uid` after provisioning.\nRepresents the file UID. Defaults to `0`."]
    pub fn file_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElDnsConfigEl {
    nameservers: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<ListField<PrimField<String>>>,
}

impl ServiceTaskSpecElContainerSpecElDnsConfigEl {
    #[doc= "Set the field `options`.\nA list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.)"]
    pub fn set_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.options = Some(v.into());
        self
    }

    #[doc= "Set the field `search`.\nA search list for host-name lookup"]
    pub fn set_search(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.search = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElDnsConfigEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElDnsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElDnsConfigEl {
    #[doc= "The IP addresses of the name servers"]
    pub nameservers: ListField<PrimField<String>>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElDnsConfigElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElDnsConfigElRef {
        ServiceTaskSpecElContainerSpecElDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElDnsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nameservers` after provisioning.\nThe IP addresses of the name servers"]
    pub fn nameservers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.nameservers", self.base))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nA list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.)"]
    pub fn options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nA search list for host-name lookup"]
    pub fn search(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElHealthcheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_period: Option<PrimField<String>>,
    test: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
}

impl ServiceTaskSpecElContainerSpecElHealthcheckEl {
    #[doc= "Set the field `interval`.\nTime between running the check (ms|s|m|h). Defaults to `0s`."]
    pub fn set_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc= "Set the field `retries`.\nConsecutive failures needed to report unhealthy. Defaults to `0`"]
    pub fn set_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retries = Some(v.into());
        self
    }

    #[doc= "Set the field `start_period`.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn set_start_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_period = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout`.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElHealthcheckEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElHealthcheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElHealthcheckEl {
    #[doc= "The test to perform as list"]
    pub test: ListField<PrimField<String>>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElHealthcheckElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElHealthcheckElRef {
        ServiceTaskSpecElContainerSpecElHealthcheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElHealthcheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nTime between running the check (ms|s|m|h). Defaults to `0s`."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc= "Get a reference to the value of field `retries` after provisioning.\nConsecutive failures needed to report unhealthy. Defaults to `0`"]
    pub fn retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }

    #[doc= "Get a reference to the value of field `start_period` after provisioning.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn start_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_period", self.base))
    }

    #[doc= "Get a reference to the value of field `test` after provisioning.\nThe test to perform as list"]
    pub fn test(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.test", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElHostsEl {
    host: PrimField<String>,
    ip: PrimField<String>,
}

impl ServiceTaskSpecElContainerSpecElHostsEl { }

impl ToListMappable for ServiceTaskSpecElContainerSpecElHostsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElHostsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElHostsEl {
    #[doc= "The name of the host"]
    pub host: PrimField<String>,
    #[doc= "The ip of the host"]
    pub ip: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElHostsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElHostsElRef {
        ServiceTaskSpecElContainerSpecElHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nThe name of the host"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nThe ip of the host"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElLabelsEl {
    label: PrimField<String>,
    value: PrimField<String>,
}

impl ServiceTaskSpecElContainerSpecElLabelsEl { }

impl ToListMappable for ServiceTaskSpecElContainerSpecElLabelsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElLabelsEl {
    #[doc= "Name of the label"]
    pub label: PrimField<String>,
    #[doc= "Value of the label"]
    pub value: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElLabelsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElLabelsElRef {
        ServiceTaskSpecElContainerSpecElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\nName of the label"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the label"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    propagation: Option<PrimField<String>>,
}

impl ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {
    #[doc= "Set the field `propagation`.\nBind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`"]
    pub fn set_propagation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.propagation = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
        ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `propagation` after provisioning.\nBind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`"]
    pub fn propagation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagation", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_bytes: Option<PrimField<f64>>,
}

impl ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
    #[doc= "Set the field `mode`.\nThe permission mode for the tmpfs mount in an integer"]
    pub fn set_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `size_bytes`.\nThe size for the tmpfs mount in bytes"]
    pub fn set_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_bytes = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
        ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe permission mode for the tmpfs mount in an integer"]
    pub fn mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `size_bytes` after provisioning.\nThe size for the tmpfs mount in bytes"]
    pub fn size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
    label: PrimField<String>,
    value: PrimField<String>,
}

impl ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl { }

impl ToListMappable for ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl {
    #[doc= "Name of the label"]
    pub label: PrimField<String>,
    #[doc= "Value of the label"]
    pub value: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
        ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\nName of the label"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the label"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElDynamic {
    labels: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_options: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_copy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl>>,
    dynamic: ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElDynamic,
}

impl ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
    #[doc= "Set the field `driver_name`.\nName of the driver to use to create the volume"]
    pub fn set_driver_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.driver_name = Some(v.into());
        self
    }

    #[doc= "Set the field `driver_options`.\nkey/value map of driver specific options"]
    pub fn set_driver_options(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.driver_options = Some(v.into());
        self
    }

    #[doc= "Set the field `no_copy`.\nPopulate volume with data from the target"]
    pub fn set_no_copy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_copy = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.labels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
        ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `driver_name` after provisioning.\nName of the driver to use to create the volume"]
    pub fn driver_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver_name", self.base))
    }

    #[doc= "Get a reference to the value of field `driver_options` after provisioning.\nkey/value map of driver specific options"]
    pub fn driver_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.driver_options", self.base))
    }

    #[doc= "Get a reference to the value of field `no_copy` after provisioning.\nPopulate volume with data from the target"]
    pub fn no_copy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_copy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElContainerSpecElMountsElDynamic {
    bind_options: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl>>,
    tmpfs_options: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl>>,
    volume_options: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElMountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    target: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bind_options: Option<Vec<ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tmpfs_options: Option<Vec<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_options: Option<Vec<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl>>,
    dynamic: ServiceTaskSpecElContainerSpecElMountsElDynamic,
}

impl ServiceTaskSpecElContainerSpecElMountsEl {
    #[doc= "Set the field `read_only`.\nWhether the mount should be read-only"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\nMount source (e.g. a volume name, a host path)"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc= "Set the field `bind_options`.\n"]
    pub fn set_bind_options(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElBindOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bind_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bind_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tmpfs_options`.\n"]
    pub fn set_tmpfs_options(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tmpfs_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tmpfs_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `volume_options`.\n"]
    pub fn set_volume_options(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volume_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElMountsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElMountsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElMountsEl {
    #[doc= "Container path"]
    pub target: PrimField<String>,
    #[doc= "The mount type"]
    pub type_: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElMountsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElMountsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElMountsElRef {
        ServiceTaskSpecElContainerSpecElMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read_only` after provisioning.\nWhether the mount should be read-only"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nMount source (e.g. a volume name, a host path)"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nContainer path"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe mount type"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `bind_options` after provisioning.\n"]
    pub fn bind_options(&self) -> ListRef<ServiceTaskSpecElContainerSpecElMountsElBindOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bind_options", self.base))
    }

    #[doc= "Get a reference to the value of field `tmpfs_options` after provisioning.\n"]
    pub fn tmpfs_options(&self) -> ListRef<ServiceTaskSpecElContainerSpecElMountsElTmpfsOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tmpfs_options", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_options` after provisioning.\n"]
    pub fn volume_options(&self) -> ListRef<ServiceTaskSpecElContainerSpecElMountsElVolumeOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volume_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry: Option<PrimField<String>>,
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
    #[doc= "Set the field `file`.\nLoad credential spec from this file"]
    pub fn set_file(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file = Some(v.into());
        self
    }

    #[doc= "Set the field `registry`.\nLoad credential spec from this value in the Windows registry"]
    pub fn set_registry(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
        ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `file` after provisioning.\nLoad credential spec from this file"]
    pub fn file(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc= "Get a reference to the value of field `registry` after provisioning.\nLoad credential spec from this value in the Windows registry"]
    pub fn registry(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<PrimField<String>>,
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
    #[doc= "Set the field `disable`.\nDisable SELinux"]
    pub fn set_disable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable = Some(v.into());
        self
    }

    #[doc= "Set the field `level`.\nSELinux level label"]
    pub fn set_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.level = Some(v.into());
        self
    }

    #[doc= "Set the field `role`.\nSELinux role label"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nSELinux type label"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `user`.\nSELinux user label"]
    pub fn set_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
        ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disable` after provisioning.\nDisable SELinux"]
    pub fn disable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable", self.base))
    }

    #[doc= "Get a reference to the value of field `level` after provisioning.\nSELinux level label"]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\nSELinux role label"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nSELinux type label"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nSELinux user label"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElContainerSpecElPrivilegesElDynamic {
    credential_spec: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl>>,
    se_linux_context: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElPrivilegesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_spec: Option<Vec<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    se_linux_context: Option<Vec<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl>>,
    dynamic: ServiceTaskSpecElContainerSpecElPrivilegesElDynamic,
}

impl ServiceTaskSpecElContainerSpecElPrivilegesEl {
    #[doc= "Set the field `credential_spec`.\n"]
    pub fn set_credential_spec(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.credential_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.credential_spec = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `se_linux_context`.\n"]
    pub fn set_se_linux_context(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.se_linux_context = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.se_linux_context = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElPrivilegesEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElPrivilegesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElPrivilegesElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElPrivilegesElRef {
        ServiceTaskSpecElContainerSpecElPrivilegesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElPrivilegesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `credential_spec` after provisioning.\n"]
    pub fn credential_spec(&self) -> ListRef<ServiceTaskSpecElContainerSpecElPrivilegesElCredentialSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credential_spec", self.base))
    }

    #[doc= "Get a reference to the value of field `se_linux_context` after provisioning.\n"]
    pub fn se_linux_context(&self) -> ListRef<ServiceTaskSpecElContainerSpecElPrivilegesElSeLinuxContextElRef> {
        ListRef::new(self.shared().clone(), format!("{}.se_linux_context", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecElSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_gid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_mode: Option<PrimField<f64>>,
    file_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uid: Option<PrimField<String>>,
    secret_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}

impl ServiceTaskSpecElContainerSpecElSecretsEl {
    #[doc= "Set the field `file_gid`.\nRepresents the file GID. Defaults to `0`"]
    pub fn set_file_gid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_gid = Some(v.into());
        self
    }

    #[doc= "Set the field `file_mode`.\nRepresents represents the FileMode of the file. Defaults to `0o444`"]
    pub fn set_file_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.file_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `file_uid`.\nRepresents the file UID. Defaults to `0`"]
    pub fn set_file_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_uid = Some(v.into());
        self
    }

    #[doc= "Set the field `secret_name`.\nName of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecElSecretsEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecElSecretsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecElSecretsEl {
    #[doc= "Represents the final filename in the filesystem"]
    pub file_name: PrimField<String>,
    #[doc= "ID of the specific secret that we're referencing"]
    pub secret_id: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElSecretsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElSecretsElRef {
        ServiceTaskSpecElContainerSpecElSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `file_gid` after provisioning.\nRepresents the file GID. Defaults to `0`"]
    pub fn file_gid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_gid", self.base))
    }

    #[doc= "Get a reference to the value of field `file_mode` after provisioning.\nRepresents represents the FileMode of the file. Defaults to `0o444`"]
    pub fn file_mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nRepresents the final filename in the filesystem"]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc= "Get a reference to the value of field `file_uid` after provisioning.\nRepresents the file UID. Defaults to `0`"]
    pub fn file_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_uid", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_id` after provisioning.\nID of the specific secret that we're referencing"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_name` after provisioning.\nName of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElContainerSpecElDynamic {
    configs: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElConfigsEl>>,
    dns_config: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElDnsConfigEl>>,
    healthcheck: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElHealthcheckEl>>,
    hosts: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElHostsEl>>,
    labels: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElLabelsEl>>,
    mounts: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElMountsEl>>,
    privileges: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElPrivilegesEl>>,
    secrets: Option<DynamicBlock<ServiceTaskSpecElContainerSpecElSecretsEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElContainerSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isolation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_grace_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_signal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sysctl: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configs: Option<Vec<ServiceTaskSpecElContainerSpecElConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_config: Option<Vec<ServiceTaskSpecElContainerSpecElDnsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthcheck: Option<Vec<ServiceTaskSpecElContainerSpecElHealthcheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts: Option<Vec<ServiceTaskSpecElContainerSpecElHostsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<ServiceTaskSpecElContainerSpecElLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mounts: Option<Vec<ServiceTaskSpecElContainerSpecElMountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privileges: Option<Vec<ServiceTaskSpecElContainerSpecElPrivilegesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Vec<ServiceTaskSpecElContainerSpecElSecretsEl>>,
    dynamic: ServiceTaskSpecElContainerSpecElDynamic,
}

impl ServiceTaskSpecElContainerSpecEl {
    #[doc= "Set the field `args`.\nArguments to the command"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }

    #[doc= "Set the field `command`.\nThe command/entrypoint to be run in the image. According to the [docker cli](https://github.com/docker/cli/blob/v20.10.7/cli/command/service/opts.go#L705) the override of the entrypoint is also passed to the `command` property and there is no `entrypoint` attribute in the `ContainerSpec` of the service."]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }

    #[doc= "Set the field `dir`.\nThe working directory for commands to run in"]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\nA list of environment variables in the form VAR=\"value\""]
    pub fn set_env(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.env = Some(v.into());
        self
    }

    #[doc= "Set the field `groups`.\nA list of additional groups that the container process will run as"]
    pub fn set_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.groups = Some(v.into());
        self
    }

    #[doc= "Set the field `hostname`.\nThe hostname to use for the container, as a valid RFC 1123 hostname"]
    pub fn set_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hostname = Some(v.into());
        self
    }

    #[doc= "Set the field `isolation`.\nIsolation technology of the containers running the service. (Windows only). Defaults to `default`."]
    pub fn set_isolation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.isolation = Some(v.into());
        self
    }

    #[doc= "Set the field `read_only`.\nMount the container's root filesystem as read only"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }

    #[doc= "Set the field `stop_grace_period`.\nAmount of time to wait for the container to terminate before forcefully removing it (ms|s|m|h). If not specified or '0s' the destroy will not check if all tasks/containers of the service terminate."]
    pub fn set_stop_grace_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stop_grace_period = Some(v.into());
        self
    }

    #[doc= "Set the field `stop_signal`.\nSignal to stop the container"]
    pub fn set_stop_signal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stop_signal = Some(v.into());
        self
    }

    #[doc= "Set the field `sysctl`.\nSysctls config (Linux only)"]
    pub fn set_sysctl(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.sysctl = Some(v.into());
        self
    }

    #[doc= "Set the field `user`.\nThe user inside the container"]
    pub fn set_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user = Some(v.into());
        self
    }

    #[doc= "Set the field `configs`.\n"]
    pub fn set_configs(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElConfigsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configs = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `dns_config`.\n"]
    pub fn set_dns_config(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElDnsConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dns_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `healthcheck`.\n"]
    pub fn set_healthcheck(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElHealthcheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.healthcheck = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.healthcheck = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `hosts`.\n"]
    pub fn set_hosts(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElHostsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hosts = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hosts = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElLabelsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.labels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `mounts`.\n"]
    pub fn set_mounts(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElMountsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mounts = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mounts = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `privileges`.\n"]
    pub fn set_privileges(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElPrivilegesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.privileges = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.privileges = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `secrets`.\n"]
    pub fn set_secrets(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecElSecretsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secrets = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceTaskSpecElContainerSpecEl {
    type O = BlockAssignable<ServiceTaskSpecElContainerSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElContainerSpecEl {
    #[doc= "The image name to use for the containers of the service, like `nginx:1.17.6`. Also use the data-source or resource of `docker_image` with the `repo_digest` or `docker_registry_image` with the `name` attribute for this, as shown in the examples."]
    pub image: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElContainerSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElContainerSpecElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElContainerSpecElRef {
        ServiceTaskSpecElContainerSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElContainerSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `args` after provisioning.\nArguments to the command"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc= "Get a reference to the value of field `command` after provisioning.\nThe command/entrypoint to be run in the image. According to the [docker cli](https://github.com/docker/cli/blob/v20.10.7/cli/command/service/opts.go#L705) the override of the entrypoint is also passed to the `command` property and there is no `entrypoint` attribute in the `ContainerSpec` of the service."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc= "Get a reference to the value of field `dir` after provisioning.\nThe working directory for commands to run in"]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nA list of environment variables in the form VAR=\"value\""]
    pub fn env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc= "Get a reference to the value of field `groups` after provisioning.\nA list of additional groups that the container process will run as"]
    pub fn groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nThe hostname to use for the container, as a valid RFC 1123 hostname"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nThe image name to use for the containers of the service, like `nginx:1.17.6`. Also use the data-source or resource of `docker_image` with the `repo_digest` or `docker_registry_image` with the `name` attribute for this, as shown in the examples."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `isolation` after provisioning.\nIsolation technology of the containers running the service. (Windows only). Defaults to `default`."]
    pub fn isolation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.isolation", self.base))
    }

    #[doc= "Get a reference to the value of field `read_only` after provisioning.\nMount the container's root filesystem as read only"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc= "Get a reference to the value of field `stop_grace_period` after provisioning.\nAmount of time to wait for the container to terminate before forcefully removing it (ms|s|m|h). If not specified or '0s' the destroy will not check if all tasks/containers of the service terminate."]
    pub fn stop_grace_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stop_grace_period", self.base))
    }

    #[doc= "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to stop the container"]
    pub fn stop_signal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.base))
    }

    #[doc= "Get a reference to the value of field `sysctl` after provisioning.\nSysctls config (Linux only)"]
    pub fn sysctl(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctl", self.base))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nThe user inside the container"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }

    #[doc= "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> ListRef<ServiceTaskSpecElContainerSpecElDnsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_config", self.base))
    }

    #[doc= "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> ListRef<ServiceTaskSpecElContainerSpecElHealthcheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.base))
    }

    #[doc= "Get a reference to the value of field `privileges` after provisioning.\n"]
    pub fn privileges(&self) -> ListRef<ServiceTaskSpecElContainerSpecElPrivilegesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.privileges", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElLogDriverEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<RecField<PrimField<String>>>,
}

impl ServiceTaskSpecElLogDriverEl {
    #[doc= "Set the field `options`.\nThe options for the logging driver"]
    pub fn set_options(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.options = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElLogDriverEl {
    type O = BlockAssignable<ServiceTaskSpecElLogDriverEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElLogDriverEl {
    #[doc= "The logging driver to use"]
    pub name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElLogDriverElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElLogDriverElRef {
        ServiceTaskSpecElLogDriverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElLogDriverElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe logging driver to use"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nThe options for the logging driver"]
    pub fn options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElNetworksAdvancedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_opts: Option<SetField<PrimField<String>>>,
    name: PrimField<String>,
}

impl ServiceTaskSpecElNetworksAdvancedEl {
    #[doc= "Set the field `aliases`.\nThe network aliases of the container in the specific network."]
    pub fn set_aliases(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.aliases = Some(v.into());
        self
    }

    #[doc= "Set the field `driver_opts`.\nAn array of driver options for the network, e.g. `opts1=value`"]
    pub fn set_driver_opts(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.driver_opts = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElNetworksAdvancedEl {
    type O = BlockAssignable<ServiceTaskSpecElNetworksAdvancedEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElNetworksAdvancedEl {
    #[doc= "The name/id of the network."]
    pub name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElNetworksAdvancedElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElNetworksAdvancedElRef {
        ServiceTaskSpecElNetworksAdvancedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElNetworksAdvancedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `aliases` after provisioning.\nThe network aliases of the container in the specific network."]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.aliases", self.base))
    }

    #[doc= "Get a reference to the value of field `driver_opts` after provisioning.\nAn array of driver options for the network, e.g. `opts1=value`"]
    pub fn driver_opts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.driver_opts", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name/id of the network."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElPlacementElPlatformsEl {
    architecture: PrimField<String>,
    os: PrimField<String>,
}

impl ServiceTaskSpecElPlacementElPlatformsEl { }

impl ToListMappable for ServiceTaskSpecElPlacementElPlatformsEl {
    type O = BlockAssignable<ServiceTaskSpecElPlacementElPlatformsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceTaskSpecElPlacementElPlatformsEl {
    #[doc= "The architecture, e.g. `amd64`"]
    pub architecture: PrimField<String>,
    #[doc= "The operation system, e.g. `linux`"]
    pub os: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElPlacementElPlatformsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElPlacementElPlatformsElRef {
        ServiceTaskSpecElPlacementElPlatformsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElPlacementElPlatformsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `architecture` after provisioning.\nThe architecture, e.g. `amd64`"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc= "Get a reference to the value of field `os` after provisioning.\nThe operation system, e.g. `linux`"]
    pub fn os(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElPlacementElDynamic {
    platforms: Option<DynamicBlock<ServiceTaskSpecElPlacementElPlatformsEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElPlacementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    constraints: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_replicas: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefs: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platforms: Option<Vec<ServiceTaskSpecElPlacementElPlatformsEl>>,
    dynamic: ServiceTaskSpecElPlacementElDynamic,
}

impl ServiceTaskSpecElPlacementEl {
    #[doc= "Set the field `constraints`.\nAn array of constraints. e.g.: `node.role==manager`"]
    pub fn set_constraints(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.constraints = Some(v.into());
        self
    }

    #[doc= "Set the field `max_replicas`.\nMaximum number of replicas for per node (default value is `0`, which is unlimited)"]
    pub fn set_max_replicas(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_replicas = Some(v.into());
        self
    }

    #[doc= "Set the field `prefs`.\nPreferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence, e.g.: `spread=node.role.manager`"]
    pub fn set_prefs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.prefs = Some(v.into());
        self
    }

    #[doc= "Set the field `platforms`.\n"]
    pub fn set_platforms(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElPlacementElPlatformsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.platforms = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.platforms = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceTaskSpecElPlacementEl {
    type O = BlockAssignable<ServiceTaskSpecElPlacementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElPlacementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElPlacementElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElPlacementElRef {
        ServiceTaskSpecElPlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElPlacementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `constraints` after provisioning.\nAn array of constraints. e.g.: `node.role==manager`"]
    pub fn constraints(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }

    #[doc= "Get a reference to the value of field `max_replicas` after provisioning.\nMaximum number of replicas for per node (default value is `0`, which is unlimited)"]
    pub fn max_replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_replicas", self.base))
    }

    #[doc= "Get a reference to the value of field `prefs` after provisioning.\nPreferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence, e.g.: `spread=node.role.manager`"]
    pub fn prefs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.prefs", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElResourcesElLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nano_cpus: Option<PrimField<f64>>,
}

impl ServiceTaskSpecElResourcesElLimitsEl {
    #[doc= "Set the field `memory_bytes`.\nThe amounf of memory in bytes the container allocates"]
    pub fn set_memory_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `nano_cpus`.\nCPU shares in units of `1/1e9` (or `10^-9`) of the CPU. Should be at least `1000000`"]
    pub fn set_nano_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nano_cpus = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElResourcesElLimitsEl {
    type O = BlockAssignable<ServiceTaskSpecElResourcesElLimitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElResourcesElLimitsElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElResourcesElLimitsElRef {
        ServiceTaskSpecElResourcesElLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElResourcesElLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `memory_bytes` after provisioning.\nThe amounf of memory in bytes the container allocates"]
    pub fn memory_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `nano_cpus` after provisioning.\nCPU shares in units of `1/1e9` (or `10^-9`) of the CPU. Should be at least `1000000`"]
    pub fn nano_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nano_cpus", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    discrete_resources_spec: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    named_resources_spec: Option<SetField<PrimField<String>>>,
}

impl ServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
    #[doc= "Set the field `discrete_resources_spec`.\nThe Integer resources"]
    pub fn set_discrete_resources_spec(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.discrete_resources_spec = Some(v.into());
        self
    }

    #[doc= "Set the field `named_resources_spec`.\nThe String resources"]
    pub fn set_named_resources_spec(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.named_resources_spec = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElResourcesElReservationElGenericResourcesEl {
    type O = BlockAssignable<ServiceTaskSpecElResourcesElReservationElGenericResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
        ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `discrete_resources_spec` after provisioning.\nThe Integer resources"]
    pub fn discrete_resources_spec(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.discrete_resources_spec", self.base))
    }

    #[doc= "Get a reference to the value of field `named_resources_spec` after provisioning.\nThe String resources"]
    pub fn named_resources_spec(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.named_resources_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElResourcesElReservationElDynamic {
    generic_resources: Option<DynamicBlock<ServiceTaskSpecElResourcesElReservationElGenericResourcesEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElResourcesElReservationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nano_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generic_resources: Option<Vec<ServiceTaskSpecElResourcesElReservationElGenericResourcesEl>>,
    dynamic: ServiceTaskSpecElResourcesElReservationElDynamic,
}

impl ServiceTaskSpecElResourcesElReservationEl {
    #[doc= "Set the field `memory_bytes`.\nThe amounf of memory in bytes the container allocates"]
    pub fn set_memory_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `nano_cpus`.\nCPU shares in units of 1/1e9 (or 10^-9) of the CPU. Should be at least `1000000`"]
    pub fn set_nano_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nano_cpus = Some(v.into());
        self
    }

    #[doc= "Set the field `generic_resources`.\n"]
    pub fn set_generic_resources(
        mut self,
        v: impl Into<BlockAssignable<ServiceTaskSpecElResourcesElReservationElGenericResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generic_resources = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generic_resources = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceTaskSpecElResourcesElReservationEl {
    type O = BlockAssignable<ServiceTaskSpecElResourcesElReservationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElResourcesElReservationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElResourcesElReservationElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElResourcesElReservationElRef {
        ServiceTaskSpecElResourcesElReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElResourcesElReservationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `memory_bytes` after provisioning.\nThe amounf of memory in bytes the container allocates"]
    pub fn memory_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `nano_cpus` after provisioning.\nCPU shares in units of 1/1e9 (or 10^-9) of the CPU. Should be at least `1000000`"]
    pub fn nano_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nano_cpus", self.base))
    }

    #[doc= "Get a reference to the value of field `generic_resources` after provisioning.\n"]
    pub fn generic_resources(&self) -> ListRef<ServiceTaskSpecElResourcesElReservationElGenericResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.generic_resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElResourcesElDynamic {
    limits: Option<DynamicBlock<ServiceTaskSpecElResourcesElLimitsEl>>,
    reservation: Option<DynamicBlock<ServiceTaskSpecElResourcesElReservationEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<Vec<ServiceTaskSpecElResourcesElLimitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation: Option<Vec<ServiceTaskSpecElResourcesElReservationEl>>,
    dynamic: ServiceTaskSpecElResourcesElDynamic,
}

impl ServiceTaskSpecElResourcesEl {
    #[doc= "Set the field `limits`.\n"]
    pub fn set_limits(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElResourcesElLimitsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.limits = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `reservation`.\n"]
    pub fn set_reservation(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElResourcesElReservationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reservation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceTaskSpecElResourcesEl {
    type O = BlockAssignable<ServiceTaskSpecElResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElResourcesElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElResourcesElRef {
        ServiceTaskSpecElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> ListRef<ServiceTaskSpecElResourcesElLimitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc= "Get a reference to the value of field `reservation` after provisioning.\n"]
    pub fn reservation(&self) -> ListRef<ServiceTaskSpecElResourcesElReservationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reservation", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceTaskSpecElRestartPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window: Option<PrimField<String>>,
}

impl ServiceTaskSpecElRestartPolicyEl {
    #[doc= "Set the field `condition`.\nCondition for restart"]
    pub fn set_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition = Some(v.into());
        self
    }

    #[doc= "Set the field `delay`.\nDelay between restart attempts (ms|s|m|h)"]
    pub fn set_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delay = Some(v.into());
        self
    }

    #[doc= "Set the field `max_attempts`.\nMaximum attempts to restart a given container before giving up (default value is `0`, which is ignored)"]
    pub fn set_max_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_attempts = Some(v.into());
        self
    }

    #[doc= "Set the field `window`.\nThe time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)"]
    pub fn set_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.window = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceTaskSpecElRestartPolicyEl {
    type O = BlockAssignable<ServiceTaskSpecElRestartPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElRestartPolicyElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElRestartPolicyElRef {
        ServiceTaskSpecElRestartPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElRestartPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `condition` after provisioning.\nCondition for restart"]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc= "Get a reference to the value of field `delay` after provisioning.\nDelay between restart attempts (ms|s|m|h)"]
    pub fn delay(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc= "Get a reference to the value of field `max_attempts` after provisioning.\nMaximum attempts to restart a given container before giving up (default value is `0`, which is ignored)"]
    pub fn max_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc= "Get a reference to the value of field `window` after provisioning.\nThe time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)"]
    pub fn window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceTaskSpecElDynamic {
    container_spec: Option<DynamicBlock<ServiceTaskSpecElContainerSpecEl>>,
    log_driver: Option<DynamicBlock<ServiceTaskSpecElLogDriverEl>>,
    networks_advanced: Option<DynamicBlock<ServiceTaskSpecElNetworksAdvancedEl>>,
    placement: Option<DynamicBlock<ServiceTaskSpecElPlacementEl>>,
    resources: Option<DynamicBlock<ServiceTaskSpecElResourcesEl>>,
    restart_policy: Option<DynamicBlock<ServiceTaskSpecElRestartPolicyEl>>,
}

#[derive(Serialize)]
pub struct ServiceTaskSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    force_update: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_spec: Option<Vec<ServiceTaskSpecElContainerSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_driver: Option<Vec<ServiceTaskSpecElLogDriverEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks_advanced: Option<Vec<ServiceTaskSpecElNetworksAdvancedEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement: Option<Vec<ServiceTaskSpecElPlacementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Vec<ServiceTaskSpecElResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_policy: Option<Vec<ServiceTaskSpecElRestartPolicyEl>>,
    dynamic: ServiceTaskSpecElDynamic,
}

impl ServiceTaskSpecEl {
    #[doc= "Set the field `force_update`.\nA counter that triggers an update even if no relevant parameters have been changed. See the [spec](https://github.com/docker/swarmkit/blob/master/api/specs.proto#L126)."]
    pub fn set_force_update(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.force_update = Some(v.into());
        self
    }

    #[doc= "Set the field `runtime`.\nRuntime is the type of runtime specified for the task executor. See the [types](https://github.com/moby/moby/blob/master/api/types/swarm/runtime.go)."]
    pub fn set_runtime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runtime = Some(v.into());
        self
    }

    #[doc= "Set the field `container_spec`.\n"]
    pub fn set_container_spec(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElContainerSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_spec = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `log_driver`.\n"]
    pub fn set_log_driver(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElLogDriverEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_driver = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_driver = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `networks_advanced`.\n"]
    pub fn set_networks_advanced(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElNetworksAdvancedEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.networks_advanced = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.networks_advanced = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `placement`.\n"]
    pub fn set_placement(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElPlacementEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.placement = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.placement = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `resources`.\n"]
    pub fn set_resources(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElResourcesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resources = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `restart_policy`.\n"]
    pub fn set_restart_policy(mut self, v: impl Into<BlockAssignable<ServiceTaskSpecElRestartPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.restart_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.restart_policy = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ServiceTaskSpecEl {
    type O = BlockAssignable<ServiceTaskSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceTaskSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceTaskSpecElRef {
    fn new(shared: StackShared, base: String) -> ServiceTaskSpecElRef {
        ServiceTaskSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceTaskSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `force_update` after provisioning.\nA counter that triggers an update even if no relevant parameters have been changed. See the [spec](https://github.com/docker/swarmkit/blob/master/api/specs.proto#L126)."]
    pub fn force_update(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_update", self.base))
    }

    #[doc= "Get a reference to the value of field `runtime` after provisioning.\nRuntime is the type of runtime specified for the task executor. See the [types](https://github.com/moby/moby/blob/master/api/types/swarm/runtime.go)."]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc= "Get a reference to the value of field `container_spec` after provisioning.\n"]
    pub fn container_spec(&self) -> ListRef<ServiceTaskSpecElContainerSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_spec", self.base))
    }

    #[doc= "Get a reference to the value of field `log_driver` after provisioning.\n"]
    pub fn log_driver(&self) -> ListRef<ServiceTaskSpecElLogDriverElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_driver", self.base))
    }

    #[doc= "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> ListRef<ServiceTaskSpecElPlacementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.placement", self.base))
    }

    #[doc= "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<ServiceTaskSpecElResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc= "Get a reference to the value of field `restart_policy` after provisioning.\n"]
    pub fn restart_policy(&self) -> ListRef<ServiceTaskSpecElRestartPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restart_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceUpdateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_failure_ratio: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism: Option<PrimField<f64>>,
}

impl ServiceUpdateConfigEl {
    #[doc= "Set the field `delay`.\nDelay between task updates `(ns|us|ms|s|m|h)`. Defaults to `0s`."]
    pub fn set_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delay = Some(v.into());
        self
    }

    #[doc= "Set the field `failure_action`.\nAction on update failure: `pause`, `continue` or `rollback`. Defaults to `pause`."]
    pub fn set_failure_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failure_action = Some(v.into());
        self
    }

    #[doc= "Set the field `max_failure_ratio`.\nFailure rate to tolerate during an update. Defaults to `0.0`."]
    pub fn set_max_failure_ratio(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_failure_ratio = Some(v.into());
        self
    }

    #[doc= "Set the field `monitor`.\nDuration after each task update to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`."]
    pub fn set_monitor(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.monitor = Some(v.into());
        self
    }

    #[doc= "Set the field `order`.\nUpdate order: either 'stop-first' or 'start-first'. Defaults to `stop-first`."]
    pub fn set_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.order = Some(v.into());
        self
    }

    #[doc= "Set the field `parallelism`.\nMaximum number of tasks to be updated in one iteration. Defaults to `1`"]
    pub fn set_parallelism(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallelism = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceUpdateConfigEl {
    type O = BlockAssignable<ServiceUpdateConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ServiceUpdateConfigElRef {
    fn new(shared: StackShared, base: String) -> ServiceUpdateConfigElRef {
        ServiceUpdateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceUpdateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delay` after provisioning.\nDelay between task updates `(ns|us|ms|s|m|h)`. Defaults to `0s`."]
    pub fn delay(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delay", self.base))
    }

    #[doc= "Get a reference to the value of field `failure_action` after provisioning.\nAction on update failure: `pause`, `continue` or `rollback`. Defaults to `pause`."]
    pub fn failure_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_action", self.base))
    }

    #[doc= "Get a reference to the value of field `max_failure_ratio` after provisioning.\nFailure rate to tolerate during an update. Defaults to `0.0`."]
    pub fn max_failure_ratio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_failure_ratio", self.base))
    }

    #[doc= "Get a reference to the value of field `monitor` after provisioning.\nDuration after each task update to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`."]
    pub fn monitor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor", self.base))
    }

    #[doc= "Get a reference to the value of field `order` after provisioning.\nUpdate order: either 'stop-first' or 'start-first'. Defaults to `stop-first`."]
    pub fn order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc= "Get a reference to the value of field `parallelism` after provisioning.\nMaximum number of tasks to be updated in one iteration. Defaults to `1`"]
    pub fn parallelism(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceDynamic {
    auth: Option<DynamicBlock<ServiceAuthEl>>,
    converge_config: Option<DynamicBlock<ServiceConvergeConfigEl>>,
    endpoint_spec: Option<DynamicBlock<ServiceEndpointSpecEl>>,
    labels: Option<DynamicBlock<ServiceLabelsEl>>,
    mode: Option<DynamicBlock<ServiceModeEl>>,
    rollback_config: Option<DynamicBlock<ServiceRollbackConfigEl>>,
    task_spec: Option<DynamicBlock<ServiceTaskSpecEl>>,
    update_config: Option<DynamicBlock<ServiceUpdateConfigEl>>,
}
