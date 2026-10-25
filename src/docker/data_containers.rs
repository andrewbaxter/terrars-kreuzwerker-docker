#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct DataContainersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
}

struct DataContainers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataContainersData_>,
}

#[derive(Clone)]
pub struct DataContainers(std::rc::Rc<DataContainers_>);

impl DataContainers {
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

    #[doc = "Get a reference to the value of field `containers` after provisioning.\nList of Docker containers."]
    pub fn containers(&self) -> terrars::ListRef<DataContainers_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this data source."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for DataContainers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataContainers { }

impl terrars::ToListMappable for DataContainers {
    type O = terrars::ListRef<DataContainersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataContainers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "docker_containers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataContainers {
    pub tf_id: std::string::String,
}

impl BuildDataContainers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataContainers {
        let out = DataContainers(std::rc::Rc::new(DataContainers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataContainersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataContainersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\nList of Docker containers."]
    pub fn containers(&self) -> terrars::ListRef<DataContainers_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this data source."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataContainers_Containers {}

impl DataContainers_Containers { }

impl terrars::ToListMappable for DataContainers_Containers {
    type O = terrars::BlockAssignable<DataContainers_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataContainers_Containers {}

impl BuildDataContainers_Containers {
    pub fn build(self) -> DataContainers_Containers {
        DataContainers_Containers {}
    }
}

#[allow(dead_code)]
pub struct DataContainers_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataContainers_ContainersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataContainers_ContainersRef {
        DataContainers_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataContainers_ContainersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nThe command used to run the container."]
    pub fn command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nThe Unix timestamp when the container was created."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe Docker container ID."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\nThe image used by the container."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\nThe image ID used by the container."]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels applied to the container."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\nThe container names."]
    pub fn names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.names", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe container state."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe container status."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
