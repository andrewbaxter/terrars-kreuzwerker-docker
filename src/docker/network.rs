#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct NetworkData_ {
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
    attachable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_config: std::option::Option<std::vec::Vec<Network_IpamConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<Network_Labels>>,
    dynamic: NetworkDynamic,
}

struct Network_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkData_>,
}

#[derive(Clone)]
pub struct Network(std::rc::Rc<Network_>);

impl Network {
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

    #[doc = "Set the field `attachable`.\nEnable manual container attachment to the network."]
    pub fn set_attachable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().attachable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `driver`.\nThe driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details."]
    pub fn set_driver(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().driver = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress`.\nCreate swarm routing-mesh network. Defaults to `false`."]
    pub fn set_ingress(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ingress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal`.\nWhether the network is internal."]
    pub fn set_internal(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().internal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_driver`.\nDriver used by the custom IP scheme of the network. Defaults to `default`"]
    pub fn set_ipam_driver(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipam_driver = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipam_options`.\nProvide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details."]
    pub fn set_ipam_options(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ipam_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\nEnable IPv6 networking. Defaults to `false`."]
    pub fn set_ipv6(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `options`.\nOnly available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details."]
    pub fn set_options(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_config`.\n"]
    pub fn set_ipam_config(self, v: impl std::convert::Into<terrars::BlockAssignable<Network_IpamConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ipam_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ipam_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl std::convert::Into<terrars::BlockAssignable<Network_Labels>>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `attachable` after provisioning.\nEnable manual container attachment to the network."]
    pub fn attachable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver` after provisioning.\nThe driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details."]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress` after provisioning.\nCreate swarm routing-mesh network. Defaults to `false`."]
    pub fn ingress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\nWhether the network is internal."]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipam_driver` after provisioning.\nDriver used by the custom IP scheme of the network. Defaults to `default`"]
    pub fn ipam_driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_driver", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipam_options` after provisioning.\nProvide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details."]
    pub fn ipam_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ipam_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6` after provisioning.\nEnable IPv6 networking. Defaults to `false`."]
    pub fn ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker network."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `options` after provisioning.\nOnly available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details."]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nScope of the network. One of `swarm`, `global`, or `local`."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

impl terrars::Referable for Network {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Network { }

impl terrars::ToListMappable for Network {
    type O = terrars::ListRef<NetworkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Network_ {
    fn extract_resource_type(&self) -> std::string::String {
        "docker_network".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetwork {
    pub tf_id: std::string::String,
    #[doc = "The name of the Docker network."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetwork {
    pub fn build(self, stack: &mut terrars::Stack) -> Network {
        let out = Network(std::rc::Rc::new(Network_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                attachable: core::default::Default::default(),
                driver: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress: core::default::Default::default(),
                internal: core::default::Default::default(),
                ipam_driver: core::default::Default::default(),
                ipam_options: core::default::Default::default(),
                ipv6: core::default::Default::default(),
                name: self.name,
                options: core::default::Default::default(),
                ipam_config: core::default::Default::default(),
                labels: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attachable` after provisioning.\nEnable manual container attachment to the network."]
    pub fn attachable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver` after provisioning.\nThe driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details."]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress` after provisioning.\nCreate swarm routing-mesh network. Defaults to `false`."]
    pub fn ingress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\nWhether the network is internal."]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipam_driver` after provisioning.\nDriver used by the custom IP scheme of the network. Defaults to `default`"]
    pub fn ipam_driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_driver", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipam_options` after provisioning.\nProvide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details."]
    pub fn ipam_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ipam_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6` after provisioning.\nEnable IPv6 networking. Defaults to `false`."]
    pub fn ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker network."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `options` after provisioning.\nOnly available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details."]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nScope of the network. One of `swarm`, `global`, or `local`."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkDynamic {
    ipam_config: std::option::Option<terrars::DynamicBlock<Network_IpamConfig>>,
    labels: std::option::Option<terrars::DynamicBlock<Network_Labels>>,
}

#[derive(Serialize)]
pub struct Network_IpamConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aux_address: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Network_IpamConfig {
    #[doc = "Set the field `aux_address`.\nAuxiliary IPv4 or IPv6 addresses used by Network driver"]
    pub fn set_aux_address(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aux_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway`.\nThe IP address of the gateway"]
    pub fn set_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_range`.\nThe ip range in CIDR form"]
    pub fn set_ip_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\nThe subnet in CIDR form"]
    pub fn set_subnet(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Network_IpamConfig {
    type O = terrars::BlockAssignable<Network_IpamConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetwork_IpamConfig {}

impl BuildNetwork_IpamConfig {
    pub fn build(self) -> Network_IpamConfig {
        Network_IpamConfig {
            aux_address: core::default::Default::default(),
            gateway: core::default::Default::default(),
            ip_range: core::default::Default::default(),
            subnet: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Network_IpamConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Network_IpamConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Network_IpamConfigRef {
        Network_IpamConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Network_IpamConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aux_address` after provisioning.\nAuxiliary IPv4 or IPv6 addresses used by Network driver"]
    pub fn aux_address(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.aux_address", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway` after provisioning.\nThe IP address of the gateway"]
    pub fn gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_range` after provisioning.\nThe ip range in CIDR form"]
    pub fn ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nThe subnet in CIDR form"]
    pub fn subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}

#[derive(Serialize)]
pub struct Network_Labels {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Network_Labels { }

impl terrars::ToListMappable for Network_Labels {
    type O = terrars::BlockAssignable<Network_Labels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetwork_Labels {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildNetwork_Labels {
    pub fn build(self) -> Network_Labels {
        Network_Labels {
            label: self.label,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Network_LabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Network_LabelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Network_LabelsRef {
        Network_LabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Network_LabelsRef {
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
