use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct NetworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_duplicate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipam_driver: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipam_options: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipam_config: Option<Vec<NetworkIpamConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<NetworkLabelsEl>>,
    dynamic: NetworkDynamic,
}

struct Network_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkData>,
}

#[derive(Clone)]
pub struct Network(Rc<Network_>);

impl Network {
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

    #[doc= "Set the field `attachable`.\nEnable manual container attachment to the network."]
    pub fn set_attachable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().attachable = Some(v.into());
        self
    }

    #[doc= "Set the field `check_duplicate`.\nRequests daemon to check for networks with same name."]
    pub fn set_check_duplicate(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().check_duplicate = Some(v.into());
        self
    }

    #[doc= "Set the field `driver`.\nThe driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details."]
    pub fn set_driver(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().driver = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ingress`.\nCreate swarm routing-mesh network. Defaults to `false`."]
    pub fn set_ingress(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ingress = Some(v.into());
        self
    }

    #[doc= "Set the field `internal`.\nWhether the network is internal."]
    pub fn set_internal(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().internal = Some(v.into());
        self
    }

    #[doc= "Set the field `ipam_driver`.\nDriver used by the custom IP scheme of the network. Defaults to `default`"]
    pub fn set_ipam_driver(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipam_driver = Some(v.into());
        self
    }

    #[doc= "Set the field `ipam_options`.\nProvide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details."]
    pub fn set_ipam_options(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ipam_options = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6`.\nEnable IPv6 networking. Defaults to `false`."]
    pub fn set_ipv6(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ipv6 = Some(v.into());
        self
    }

    #[doc= "Set the field `options`.\nOnly available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details."]
    pub fn set_options(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().options = Some(v.into());
        self
    }

    #[doc= "Set the field `ipam_config`.\n"]
    pub fn set_ipam_config(self, v: impl Into<BlockAssignable<NetworkIpamConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ipam_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ipam_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl Into<BlockAssignable<NetworkLabelsEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `attachable` after provisioning.\nEnable manual container attachment to the network."]
    pub fn attachable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check_duplicate` after provisioning.\nRequests daemon to check for networks with same name."]
    pub fn check_duplicate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.check_duplicate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `driver` after provisioning.\nThe driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details."]
    pub fn driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ingress` after provisioning.\nCreate swarm routing-mesh network. Defaults to `false`."]
    pub fn ingress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal` after provisioning.\nWhether the network is internal."]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipam_driver` after provisioning.\nDriver used by the custom IP scheme of the network. Defaults to `default`"]
    pub fn ipam_driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipam_options` after provisioning.\nProvide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details."]
    pub fn ipam_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ipam_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\nEnable IPv6 networking. Defaults to `false`."]
    pub fn ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
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

impl Resource for Network {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for Network {
    type O = ListRef<NetworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Network_ {
    fn extract_resource_type(&self) -> String {
        "docker_network".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetwork {
    pub tf_id: String,
    #[doc= "The name of the Docker network."]
    pub name: PrimField<String>,
}

impl BuildNetwork {
    pub fn build(self, stack: &mut Stack) -> Network {
        let out = Network(Rc::new(Network_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                attachable: core::default::Default::default(),
                check_duplicate: core::default::Default::default(),
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attachable` after provisioning.\nEnable manual container attachment to the network."]
    pub fn attachable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check_duplicate` after provisioning.\nRequests daemon to check for networks with same name."]
    pub fn check_duplicate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.check_duplicate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `driver` after provisioning.\nThe driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details."]
    pub fn driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ingress` after provisioning.\nCreate swarm routing-mesh network. Defaults to `false`."]
    pub fn ingress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal` after provisioning.\nWhether the network is internal."]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipam_driver` after provisioning.\nDriver used by the custom IP scheme of the network. Defaults to `default`"]
    pub fn ipam_driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipam_options` after provisioning.\nProvide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details."]
    pub fn ipam_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ipam_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\nEnable IPv6 networking. Defaults to `false`."]
    pub fn ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
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
pub struct NetworkIpamConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aux_address: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<PrimField<String>>,
}

impl NetworkIpamConfigEl {
    #[doc= "Set the field `aux_address`.\nAuxiliary IPv4 or IPv6 addresses used by Network driver"]
    pub fn set_aux_address(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.aux_address = Some(v.into());
        self
    }

    #[doc= "Set the field `gateway`.\nThe IP address of the gateway"]
    pub fn set_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_range`.\nThe ip range in CIDR form"]
    pub fn set_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_range = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet`.\nThe subnet in CIDR form"]
    pub fn set_subnet(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkIpamConfigEl {
    type O = BlockAssignable<NetworkIpamConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkIpamConfigEl {}

impl BuildNetworkIpamConfigEl {
    pub fn build(self) -> NetworkIpamConfigEl {
        NetworkIpamConfigEl {
            aux_address: core::default::Default::default(),
            gateway: core::default::Default::default(),
            ip_range: core::default::Default::default(),
            subnet: core::default::Default::default(),
        }
    }
}

pub struct NetworkIpamConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkIpamConfigElRef {
    fn new(shared: StackShared, base: String) -> NetworkIpamConfigElRef {
        NetworkIpamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkIpamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `aux_address` after provisioning.\nAuxiliary IPv4 or IPv6 addresses used by Network driver"]
    pub fn aux_address(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.aux_address", self.base))
    }

    #[doc= "Get a reference to the value of field `gateway` after provisioning.\nThe IP address of the gateway"]
    pub fn gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_range` after provisioning.\nThe ip range in CIDR form"]
    pub fn ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet` after provisioning.\nThe subnet in CIDR form"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkLabelsEl {
    label: PrimField<String>,
    value: PrimField<String>,
}

impl NetworkLabelsEl { }

impl ToListMappable for NetworkLabelsEl {
    type O = BlockAssignable<NetworkLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkLabelsEl {
    #[doc= "Name of the label"]
    pub label: PrimField<String>,
    #[doc= "Value of the label"]
    pub value: PrimField<String>,
}

impl BuildNetworkLabelsEl {
    pub fn build(self) -> NetworkLabelsEl {
        NetworkLabelsEl {
            label: self.label,
            value: self.value,
        }
    }
}

pub struct NetworkLabelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkLabelsElRef {
    fn new(shared: StackShared, base: String) -> NetworkLabelsElRef {
        NetworkLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkLabelsElRef {
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
struct NetworkDynamic {
    ipam_config: Option<DynamicBlock<NetworkIpamConfigEl>>,
    labels: Option<DynamicBlock<NetworkLabelsEl>>,
}
