#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataNetworkData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

struct DataNetwork_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkData_>,
}

#[derive(Clone)]
pub struct DataNetwork(std::rc::Rc<DataNetwork_>);

impl DataNetwork {
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

    #[doc =
        "Get a reference to the value of field `containers` after provisioning.\nContainers attached to the network."]
    pub fn containers(&self) -> terrars::ListRef<DataNetwork_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `internal` after provisioning.\nIf `true`, the network is internal."]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_config` after provisioning.\nThe IPAM configuration options"]
    pub fn ipam_config(&self) -> terrars::SetRef<DataNetwork_IpamConfigRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipam_config", self.extract_ref()))
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

impl terrars::Referable for DataNetwork {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetwork { }

impl terrars::ToListMappable for DataNetwork {
    type O = terrars::ListRef<DataNetworkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetwork_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "docker_network".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetwork {
    pub tf_id: std::string::String,
    #[doc = "The name of the Docker network."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataNetwork {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetwork {
        let out = DataNetwork(std::rc::Rc::new(DataNetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNetworkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `containers` after provisioning.\nContainers attached to the network."]
    pub fn containers(&self) -> terrars::ListRef<DataNetwork_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `internal` after provisioning.\nIf `true`, the network is internal."]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_config` after provisioning.\nThe IPAM configuration options"]
    pub fn ipam_config(&self) -> terrars::SetRef<DataNetwork_IpamConfigRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipam_config", self.extract_ref()))
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

#[derive(Serialize)]
pub struct DataNetwork_Containers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mac_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetwork_Containers {
    #[doc = "Set the field `container_id`.\n"]
    pub fn set_container_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_address`.\n"]
    pub fn set_ipv4_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mac_address`.\n"]
    pub fn set_mac_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mac_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetwork_Containers {
    type O = terrars::BlockAssignable<DataNetwork_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetwork_Containers {}

impl BuildDataNetwork_Containers {
    pub fn build(self) -> DataNetwork_Containers {
        DataNetwork_Containers {
            container_id: core::default::Default::default(),
            endpoint_id: core::default::Default::default(),
            ipv4_address: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            mac_address: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetwork_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetwork_ContainersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetwork_ContainersRef {
        DataNetwork_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetwork_ContainersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_id` after provisioning.\n"]
    pub fn container_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_id", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc = "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mac_address", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetwork_IpamConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aux_address: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetwork_IpamConfig {
    #[doc = "Set the field `aux_address`.\n"]
    pub fn set_aux_address(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aux_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway`.\n"]
    pub fn set_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_range`.\n"]
    pub fn set_ip_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetwork_IpamConfig {
    type O = terrars::BlockAssignable<DataNetwork_IpamConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetwork_IpamConfig {}

impl BuildDataNetwork_IpamConfig {
    pub fn build(self) -> DataNetwork_IpamConfig {
        DataNetwork_IpamConfig {
            aux_address: core::default::Default::default(),
            gateway: core::default::Default::default(),
            ip_range: core::default::Default::default(),
            subnet: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetwork_IpamConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetwork_IpamConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetwork_IpamConfigRef {
        DataNetwork_IpamConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetwork_IpamConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aux_address` after provisioning.\n"]
    pub fn aux_address(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.aux_address", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_range` after provisioning.\n"]
    pub fn ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}
