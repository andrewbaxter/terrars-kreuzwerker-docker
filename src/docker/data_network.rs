use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataNetworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
}

struct DataNetwork_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkData>,
}

#[derive(Clone)]
pub struct DataNetwork(Rc<DataNetwork_>);

impl DataNetwork {
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

    #[doc= "Get a reference to the value of field `driver` after provisioning.\nThe driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details."]
    pub fn driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal` after provisioning.\nIf `true`, the network is internal."]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipam_config` after provisioning.\nThe IPAM configuration options"]
    pub fn ipam_config(&self) -> SetRef<DataNetworkIpamConfigElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ipam_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker network."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nOnly available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details."]
    pub fn options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nScope of the network. One of `swarm`, `global`, or `local`."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

impl Referable for DataNetwork {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataNetwork { }

impl ToListMappable for DataNetwork {
    type O = ListRef<DataNetworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataNetwork_ {
    fn extract_datasource_type(&self) -> String {
        "docker_network".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetwork {
    pub tf_id: String,
    #[doc= "The name of the Docker network."]
    pub name: PrimField<String>,
}

impl BuildDataNetwork {
    pub fn build(self, stack: &mut Stack) -> DataNetwork {
        let out = DataNetwork(Rc::new(DataNetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNetworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `driver` after provisioning.\nThe driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details."]
    pub fn driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal` after provisioning.\nIf `true`, the network is internal."]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipam_config` after provisioning.\nThe IPAM configuration options"]
    pub fn ipam_config(&self) -> SetRef<DataNetworkIpamConfigElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ipam_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker network."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nOnly available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details."]
    pub fn options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nScope of the network. One of `swarm`, `global`, or `local`."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkIpamConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aux_address: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<PrimField<String>>,
}

impl DataNetworkIpamConfigEl {
    #[doc= "Set the field `aux_address`.\n"]
    pub fn set_aux_address(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.aux_address = Some(v.into());
        self
    }

    #[doc= "Set the field `gateway`.\n"]
    pub fn set_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_range`.\n"]
    pub fn set_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_range = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet`.\n"]
    pub fn set_subnet(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkIpamConfigEl {
    type O = BlockAssignable<DataNetworkIpamConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkIpamConfigEl {}

impl BuildDataNetworkIpamConfigEl {
    pub fn build(self) -> DataNetworkIpamConfigEl {
        DataNetworkIpamConfigEl {
            aux_address: core::default::Default::default(),
            gateway: core::default::Default::default(),
            ip_range: core::default::Default::default(),
            subnet: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkIpamConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkIpamConfigElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkIpamConfigElRef {
        DataNetworkIpamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkIpamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `aux_address` after provisioning.\n"]
    pub fn aux_address(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.aux_address", self.base))
    }

    #[doc= "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_range` after provisioning.\n"]
    pub fn ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}
