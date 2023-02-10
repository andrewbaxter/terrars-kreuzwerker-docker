use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct SecretData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<SecretLabelsEl>>,
    dynamic: SecretDynamic,
}

struct Secret_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecretData>,
}

#[derive(Clone)]
pub struct Secret(Rc<Secret_>);

impl Secret {
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

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl Into<BlockAssignable<SecretLabelsEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `data` after provisioning.\nBase64-url-safe-encoded secret data"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nUser-defined name of the secret"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Resource for Secret {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Secret {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Secret {
    type O = ListRef<SecretRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for Secret_ {
    fn extract_resource_type(&self) -> String {
        "docker_secret".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecret {
    pub tf_id: String,
    #[doc= "Base64-url-safe-encoded secret data"]
    pub data: PrimField<String>,
    #[doc= "User-defined name of the secret"]
    pub name: PrimField<String>,
}

impl BuildSecret {
    pub fn build(self, stack: &mut Stack) -> Secret {
        let out = Secret(Rc::new(Secret_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecretData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data: self.data,
                id: core::default::Default::default(),
                name: self.name,
                labels: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecretRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecretRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecretRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\nBase64-url-safe-encoded secret data"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nUser-defined name of the secret"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SecretLabelsEl {
    label: PrimField<String>,
    value: PrimField<String>,
}

impl SecretLabelsEl { }

impl ToListMappable for SecretLabelsEl {
    type O = BlockAssignable<SecretLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretLabelsEl {
    #[doc= "Name of the label"]
    pub label: PrimField<String>,
    #[doc= "Value of the label"]
    pub value: PrimField<String>,
}

impl BuildSecretLabelsEl {
    pub fn build(self) -> SecretLabelsEl {
        SecretLabelsEl {
            label: self.label,
            value: self.value,
        }
    }
}

pub struct SecretLabelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecretLabelsElRef {
    fn new(shared: StackShared, base: String) -> SecretLabelsElRef {
        SecretLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretLabelsElRef {
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
struct SecretDynamic {
    labels: Option<DynamicBlock<SecretLabelsEl>>,
}
