#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct VolumeData_ {
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
    driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_opts: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<std::vec::Vec<Volume_Cluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<Volume_Labels>>,
    dynamic: VolumeDynamic,
}

struct Volume_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VolumeData_>,
}

#[derive(Clone)]
pub struct Volume(std::rc::Rc<Volume_>);

impl Volume {
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

    #[doc = "Set the field `driver`.\nDriver type for the volume. Defaults to `local`."]
    pub fn set_driver(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().driver = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver_opts`.\nOptions specific to the driver."]
    pub fn set_driver_opts(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().driver_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the Docker volume (will be generated if not provided)."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(self, v: impl std::convert::Into<terrars::BlockAssignable<Volume_Cluster>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl std::convert::Into<terrars::BlockAssignable<Volume_Labels>>) -> Self {
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
        "Get a reference to the value of field `driver` after provisioning.\nDriver type for the volume. Defaults to `local`."]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `driver_opts` after provisioning.\nOptions specific to the driver."]
    pub fn driver_opts(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_opts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mountpoint` after provisioning.\nThe mountpoint of the volume."]
    pub fn mountpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mountpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker volume (will be generated if not provided)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::ListRef<Volume_ClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }
}

impl terrars::Referable for Volume {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Volume { }

impl terrars::ToListMappable for Volume {
    type O = terrars::ListRef<VolumeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Volume_ {
    fn extract_resource_type(&self) -> std::string::String {
        "docker_volume".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVolume {
    pub tf_id: std::string::String,
}

impl BuildVolume {
    pub fn build(self, stack: &mut terrars::Stack) -> Volume {
        let out = Volume(std::rc::Rc::new(Volume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VolumeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                driver: core::default::Default::default(),
                driver_opts: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                cluster: core::default::Default::default(),
                labels: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VolumeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VolumeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver` after provisioning.\nDriver type for the volume. Defaults to `local`."]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `driver_opts` after provisioning.\nOptions specific to the driver."]
    pub fn driver_opts(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_opts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mountpoint` after provisioning.\nThe mountpoint of the volume."]
    pub fn mountpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mountpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker volume (will be generated if not provided)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::ListRef<Volume_ClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VolumeDynamic {
    cluster: std::option::Option<terrars::DynamicBlock<Volume_Cluster>>,
    labels: std::option::Option<terrars::DynamicBlock<Volume_Labels>>,
}

#[derive(Serialize)]
pub struct Volume_Cluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sharing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topology_preferred: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topology_required: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Volume_Cluster {
    #[doc =
        "Set the field `availability`.\nAvailability of the volume. Can be `active` (default), `pause`, or `drain`."]
    pub fn set_availability(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\nCluster Volume group"]
    pub fn set_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `limit_bytes`.\nMinimum size of the Cluster Volume in human readable memory bytes (like 128MiB, 2GiB, etc). Must be in format of KiB, MiB, Gib, Tib or PiB."]
    pub fn set_limit_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.limit_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `required_bytes`.\nMaximum size of the Cluster Volume in human readable memory bytes (like 128MiB, 2GiB, etc). Must be in format of KiB, MiB, Gib, Tib or PiB."]
    pub fn set_required_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.required_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nThe scope of the volume. Can be `single` (default) or `multi`."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sharing`.\nThe sharing mode. Can be `none` (default), `readonly`, `onewriter` or `all`."]
    pub fn set_sharing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sharing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topology_preferred`.\nA topology that the Cluster Volume would be preferred in"]
    pub fn set_topology_preferred(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.topology_preferred = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topology_required`.\nA topology that the Cluster Volume must be accessible from"]
    pub fn set_topology_required(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topology_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nCluster Volume access type. Can be `mount` or `block` (default)."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Volume_Cluster {
    type O = terrars::BlockAssignable<Volume_Cluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVolume_Cluster {}

impl BuildVolume_Cluster {
    pub fn build(self) -> Volume_Cluster {
        Volume_Cluster {
            availability: core::default::Default::default(),
            group: core::default::Default::default(),
            limit_bytes: core::default::Default::default(),
            required_bytes: core::default::Default::default(),
            scope: core::default::Default::default(),
            sharing: core::default::Default::default(),
            topology_preferred: core::default::Default::default(),
            topology_required: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Volume_ClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Volume_ClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Volume_ClusterRef {
        Volume_ClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Volume_ClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `availability` after provisioning.\nAvailability of the volume. Can be `active` (default), `pause`, or `drain`."]
    pub fn availability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\nCluster Volume group"]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the cluster volume."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limit_bytes` after provisioning.\nMinimum size of the Cluster Volume in human readable memory bytes (like 128MiB, 2GiB, etc). Must be in format of KiB, MiB, Gib, Tib or PiB."]
    pub fn limit_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required_bytes` after provisioning.\nMaximum size of the Cluster Volume in human readable memory bytes (like 128MiB, 2GiB, etc). Must be in format of KiB, MiB, Gib, Tib or PiB."]
    pub fn required_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe scope of the volume. Can be `single` (default) or `multi`."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sharing` after provisioning.\nThe sharing mode. Can be `none` (default), `readonly`, `onewriter` or `all`."]
    pub fn sharing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sharing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topology_preferred` after provisioning.\nA topology that the Cluster Volume would be preferred in"]
    pub fn topology_preferred(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topology_preferred", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topology_required` after provisioning.\nA topology that the Cluster Volume must be accessible from"]
    pub fn topology_required(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topology_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nCluster Volume access type. Can be `mount` or `block` (default)."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Volume_Labels {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Volume_Labels { }

impl terrars::ToListMappable for Volume_Labels {
    type O = terrars::BlockAssignable<Volume_Labels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVolume_Labels {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildVolume_Labels {
    pub fn build(self) -> Volume_Labels {
        Volume_Labels {
            label: self.label,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Volume_LabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Volume_LabelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Volume_LabelsRef {
        Volume_LabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Volume_LabelsRef {
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
