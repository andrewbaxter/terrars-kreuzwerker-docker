use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct VolumeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_opts: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<VolumeLabelsEl>>,
    dynamic: VolumeDynamic,
}

struct Volume_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VolumeData>,
}

#[derive(Clone)]
pub struct Volume(Rc<Volume_>);

impl Volume {
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

    #[doc= "Set the field `driver`.\nDriver type for the volume. Defaults to `local`."]
    pub fn set_driver(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().driver = Some(v.into());
        self
    }

    #[doc= "Set the field `driver_opts`.\nOptions specific to the driver."]
    pub fn set_driver_opts(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().driver_opts = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of the Docker volume (will be generated if not provided)."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl Into<BlockAssignable<VolumeLabelsEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `driver` after provisioning.\nDriver type for the volume. Defaults to `local`."]
    pub fn driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `driver_opts` after provisioning.\nOptions specific to the driver."]
    pub fn driver_opts(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.driver_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mountpoint` after provisioning.\nThe mountpoint of the volume."]
    pub fn mountpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mountpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker volume (will be generated if not provided)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Resource for Volume {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for Volume {
    type O = ListRef<VolumeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Volume_ {
    fn extract_resource_type(&self) -> String {
        "docker_volume".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVolume {
    pub tf_id: String,
}

impl BuildVolume {
    pub fn build(self, stack: &mut Stack) -> Volume {
        let out = Volume(Rc::new(Volume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VolumeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                driver: core::default::Default::default(),
                driver_opts: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                labels: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VolumeRef {
    shared: StackShared,
    base: String,
}

impl Ref for VolumeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VolumeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `driver` after provisioning.\nDriver type for the volume. Defaults to `local`."]
    pub fn driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `driver_opts` after provisioning.\nOptions specific to the driver."]
    pub fn driver_opts(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.driver_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mountpoint` after provisioning.\nThe mountpoint of the volume."]
    pub fn mountpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mountpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker volume (will be generated if not provided)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VolumeLabelsEl {
    label: PrimField<String>,
    value: PrimField<String>,
}

impl VolumeLabelsEl { }

impl ToListMappable for VolumeLabelsEl {
    type O = BlockAssignable<VolumeLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVolumeLabelsEl {
    #[doc= "Name of the label"]
    pub label: PrimField<String>,
    #[doc= "Value of the label"]
    pub value: PrimField<String>,
}

impl BuildVolumeLabelsEl {
    pub fn build(self) -> VolumeLabelsEl {
        VolumeLabelsEl {
            label: self.label,
            value: self.value,
        }
    }
}

pub struct VolumeLabelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VolumeLabelsElRef {
    fn new(shared: StackShared, base: String) -> VolumeLabelsElRef {
        VolumeLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VolumeLabelsElRef {
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
struct VolumeDynamic {
    labels: Option<DynamicBlock<VolumeLabelsEl>>,
}
