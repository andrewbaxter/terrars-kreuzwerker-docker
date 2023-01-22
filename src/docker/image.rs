use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct ImageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_remove: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_locally: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_triggers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    triggers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build: Option<Vec<ImageBuildEl>>,
    dynamic: ImageDynamic,
}

struct Image_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImageData>,
}

#[derive(Clone)]
pub struct Image(Rc<Image_>);

impl Image {
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

    #[doc= "Set the field `force_remove`.\nIf true, then the image is removed forcibly when the resource is destroyed."]
    pub fn set_force_remove(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `keep_locally`.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation."]
    pub fn set_keep_locally(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().keep_locally = Some(v.into());
        self
    }

    #[doc= "Set the field `platform`.\nThe platform to use when pulling the image. Defaults to the platform of the current machine."]
    pub fn set_platform(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().platform = Some(v.into());
        self
    }

    #[doc= "Set the field `pull_triggers`.\nList of values which cause an image pull when changed. This is used to store the image digest from the registry when using the [docker_registry_image](../data-sources/registry_image.md)."]
    pub fn set_pull_triggers(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().pull_triggers = Some(v.into());
        self
    }

    #[doc= "Set the field `triggers`.\nA map of arbitrary strings that, when changed, will force the `docker_image` resource to be replaced. This can be used to rebuild an image when contents of source code folders change"]
    pub fn set_triggers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().triggers = Some(v.into());
        self
    }

    #[doc= "Set the field `build`.\n"]
    pub fn set_build(self, v: impl Into<BlockAssignable<ImageBuildEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `force_remove` after provisioning.\nIf true, then the image is removed forcibly when the resource is destroyed."]
    pub fn force_remove(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_remove", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for this resource. This is not the image ID, but the ID of the resource in the Terraform state. This is used to identify the resource in the Terraform state. To reference the correct image ID, use the `image_id` attribute."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_id` after provisioning.\nThe ID of the image (as seen when executing `docker inspect` on the image). Can be used to reference the image via its ID in other resources."]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keep_locally` after provisioning.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation."]
    pub fn keep_locally(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_locally", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image, including any tags or SHA256 repo digests."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform` after provisioning.\nThe platform to use when pulling the image. Defaults to the platform of the current machine."]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pull_triggers` after provisioning.\nList of values which cause an image pull when changed. This is used to store the image digest from the registry when using the [docker_registry_image](../data-sources/registry_image.md)."]
    pub fn pull_triggers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.pull_triggers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_digest` after provisioning.\nThe image sha256 digest in the form of `repo[:tag]@sha256:<hash>`."]
    pub fn repo_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_digest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `triggers` after provisioning.\nA map of arbitrary strings that, when changed, will force the `docker_image` resource to be replaced. This can be used to rebuild an image when contents of source code folders change"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}

impl Resource for Image {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for Image {
    type O = ListRef<ImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Image_ {
    fn extract_resource_type(&self) -> String {
        "docker_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImage {
    pub tf_id: String,
    #[doc= "The name of the Docker image, including any tags or SHA256 repo digests."]
    pub name: PrimField<String>,
}

impl BuildImage {
    pub fn build(self, stack: &mut Stack) -> Image {
        let out = Image(Rc::new(Image_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                force_remove: core::default::Default::default(),
                keep_locally: core::default::Default::default(),
                name: self.name,
                platform: core::default::Default::default(),
                pull_triggers: core::default::Default::default(),
                triggers: core::default::Default::default(),
                build: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImageRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImageRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `force_remove` after provisioning.\nIf true, then the image is removed forcibly when the resource is destroyed."]
    pub fn force_remove(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_remove", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for this resource. This is not the image ID, but the ID of the resource in the Terraform state. This is used to identify the resource in the Terraform state. To reference the correct image ID, use the `image_id` attribute."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_id` after provisioning.\nThe ID of the image (as seen when executing `docker inspect` on the image). Can be used to reference the image via its ID in other resources."]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keep_locally` after provisioning.\nIf true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation."]
    pub fn keep_locally(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_locally", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Docker image, including any tags or SHA256 repo digests."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform` after provisioning.\nThe platform to use when pulling the image. Defaults to the platform of the current machine."]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pull_triggers` after provisioning.\nList of values which cause an image pull when changed. This is used to store the image digest from the registry when using the [docker_registry_image](../data-sources/registry_image.md)."]
    pub fn pull_triggers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.pull_triggers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_digest` after provisioning.\nThe image sha256 digest in the form of `repo[:tag]@sha256:<hash>`."]
    pub fn repo_digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_digest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `triggers` after provisioning.\nA map of arbitrary strings that, when changed, will force the `docker_image` resource to be replaced. This can be used to rebuild an image when contents of source code folders change"]
    pub fn triggers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.triggers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImageBuildElAuthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    host_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<PrimField<String>>,
}

impl ImageBuildElAuthConfigEl {
    #[doc= "Set the field `auth`.\nthe auth token"]
    pub fn set_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auth = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\nthe user emal"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_token`.\nthe identity token"]
    pub fn set_identity_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_token = Some(v.into());
        self
    }

    #[doc= "Set the field `password`.\nthe registry password"]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }

    #[doc= "Set the field `registry_token`.\nthe registry token"]
    pub fn set_registry_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry_token = Some(v.into());
        self
    }

    #[doc= "Set the field `server_address`.\nthe server address"]
    pub fn set_server_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_address = Some(v.into());
        self
    }

    #[doc= "Set the field `user_name`.\nthe registry user name"]
    pub fn set_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_name = Some(v.into());
        self
    }
}

impl ToListMappable for ImageBuildElAuthConfigEl {
    type O = BlockAssignable<ImageBuildElAuthConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImageBuildElAuthConfigEl {
    #[doc= "hostname of the registry"]
    pub host_name: PrimField<String>,
}

impl BuildImageBuildElAuthConfigEl {
    pub fn build(self) -> ImageBuildElAuthConfigEl {
        ImageBuildElAuthConfigEl {
            auth: core::default::Default::default(),
            email: core::default::Default::default(),
            host_name: self.host_name,
            identity_token: core::default::Default::default(),
            password: core::default::Default::default(),
            registry_token: core::default::Default::default(),
            server_address: core::default::Default::default(),
            user_name: core::default::Default::default(),
        }
    }
}

pub struct ImageBuildElAuthConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImageBuildElAuthConfigElRef {
    fn new(shared: StackShared, base: String) -> ImageBuildElAuthConfigElRef {
        ImageBuildElAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImageBuildElAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auth` after provisioning.\nthe auth token"]
    pub fn auth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nthe user emal"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `host_name` after provisioning.\nhostname of the registry"]
    pub fn host_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_token` after provisioning.\nthe identity token"]
    pub fn identity_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_token", self.base))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nthe registry password"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_token` after provisioning.\nthe registry token"]
    pub fn registry_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_token", self.base))
    }

    #[doc= "Get a reference to the value of field `server_address` after provisioning.\nthe server address"]
    pub fn server_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_address", self.base))
    }

    #[doc= "Get a reference to the value of field `user_name` after provisioning.\nthe registry user name"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ImageBuildElUlimitEl {
    hard: PrimField<f64>,
    name: PrimField<String>,
    soft: PrimField<f64>,
}

impl ImageBuildElUlimitEl { }

impl ToListMappable for ImageBuildElUlimitEl {
    type O = BlockAssignable<ImageBuildElUlimitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImageBuildElUlimitEl {
    #[doc= "soft limit"]
    pub hard: PrimField<f64>,
    #[doc= "type of ulimit, e.g. `nofile`"]
    pub name: PrimField<String>,
    #[doc= "hard limit"]
    pub soft: PrimField<f64>,
}

impl BuildImageBuildElUlimitEl {
    pub fn build(self) -> ImageBuildElUlimitEl {
        ImageBuildElUlimitEl {
            hard: self.hard,
            name: self.name,
            soft: self.soft,
        }
    }
}

pub struct ImageBuildElUlimitElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImageBuildElUlimitElRef {
    fn new(shared: StackShared, base: String) -> ImageBuildElUlimitElRef {
        ImageBuildElUlimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImageBuildElUlimitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hard` after provisioning.\nsoft limit"]
    pub fn hard(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hard", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\ntype of ulimit, e.g. `nofile`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `soft` after provisioning.\nhard limit"]
    pub fn soft(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.soft", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImageBuildElDynamic {
    auth_config: Option<DynamicBlock<ImageBuildElAuthConfigEl>>,
    ulimit: Option<DynamicBlock<ImageBuildElUlimitEl>>,
}

#[derive(Serialize)]
pub struct ImageBuildEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_arg: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_args: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_from: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroup_parent: Option<PrimField<String>>,
    context: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_quota: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_set_cpus: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_set_mems: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_shares: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_hosts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_remove: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isolation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_swap: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_cache: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_parent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_context: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_opt: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shm_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suppress_output: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_config: Option<Vec<ImageBuildElAuthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ulimit: Option<Vec<ImageBuildElUlimitEl>>,
    dynamic: ImageBuildElDynamic,
}

impl ImageBuildEl {
    #[doc= "Set the field `build_arg`.\nSet build-time variables"]
    pub fn set_build_arg(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.build_arg = Some(v.into());
        self
    }

    #[doc= "Set the field `build_args`.\nPairs for build-time variables in the form TODO"]
    pub fn set_build_args(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.build_args = Some(v.into());
        self
    }

    #[doc= "Set the field `build_id`.\nBuildID is an optional identifier that can be passed together with the build request. The same identifier can be used to gracefully cancel the build with the cancel request."]
    pub fn set_build_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_id = Some(v.into());
        self
    }

    #[doc= "Set the field `cache_from`.\nImages to consider as cache sources"]
    pub fn set_cache_from(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.cache_from = Some(v.into());
        self
    }

    #[doc= "Set the field `cgroup_parent`.\nOptional parent cgroup for the container"]
    pub fn set_cgroup_parent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cgroup_parent = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_period`.\nThe length of a CPU period in microseconds"]
    pub fn set_cpu_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_period = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_quota`.\nMicroseconds of CPU time that the container can get in a CPU period"]
    pub fn set_cpu_quota(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_quota = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_set_cpus`.\nCPUs in which to allow execution (e.g., `0-3`, `0`, `1`)"]
    pub fn set_cpu_set_cpus(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_set_cpus = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_set_mems`.\nMEMs in which to allow execution (`0-3`, `0`, `1`)"]
    pub fn set_cpu_set_mems(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_set_mems = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_shares`.\nCPU shares (relative weight)"]
    pub fn set_cpu_shares(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_shares = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile`.\nName of the Dockerfile. Defaults to `Dockerfile`."]
    pub fn set_dockerfile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile = Some(v.into());
        self
    }

    #[doc= "Set the field `extra_hosts`.\nA list of hostnames/IP mappings to add to the container’s /etc/hosts file. Specified in the form [\"hostname:IP\"]"]
    pub fn set_extra_hosts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.extra_hosts = Some(v.into());
        self
    }

    #[doc= "Set the field `force_remove`.\nAlways remove intermediate containers"]
    pub fn set_force_remove(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.force_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `isolation`.\nIsolation represents the isolation technology of a container. The supported values are "]
    pub fn set_isolation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.isolation = Some(v.into());
        self
    }

    #[doc= "Set the field `label`.\nSet metadata for an image"]
    pub fn set_label(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.label = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nUser-defined key/value metadata"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `memory`.\nSet memory limit for build"]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }

    #[doc= "Set the field `memory_swap`.\nTotal memory (memory + swap), -1 to enable unlimited swap"]
    pub fn set_memory_swap(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_swap = Some(v.into());
        self
    }

    #[doc= "Set the field `network_mode`.\nSet the networking mode for the RUN instructions during build"]
    pub fn set_network_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `no_cache`.\nDo not use the cache when building the image"]
    pub fn set_no_cache(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_cache = Some(v.into());
        self
    }

    #[doc= "Set the field `platform`.\nSet platform if server is multi-platform capable"]
    pub fn set_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.platform = Some(v.into());
        self
    }

    #[doc= "Set the field `pull_parent`.\nAttempt to pull the image even if an older image exists locally"]
    pub fn set_pull_parent(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pull_parent = Some(v.into());
        self
    }

    #[doc= "Set the field `remote_context`.\nA Git repository URI or HTTP/HTTPS context URI"]
    pub fn set_remote_context(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.remote_context = Some(v.into());
        self
    }

    #[doc= "Set the field `remove`.\nRemove intermediate containers after a successful build. Defaults to `true`."]
    pub fn set_remove(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.remove = Some(v.into());
        self
    }

    #[doc= "Set the field `security_opt`.\nThe security options"]
    pub fn set_security_opt(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.security_opt = Some(v.into());
        self
    }

    #[doc= "Set the field `session_id`.\nSet an ID for the build session"]
    pub fn set_session_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_id = Some(v.into());
        self
    }

    #[doc= "Set the field `shm_size`.\nSize of /dev/shm in bytes. The size must be greater than 0"]
    pub fn set_shm_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shm_size = Some(v.into());
        self
    }

    #[doc= "Set the field `squash`.\nIf true the new layers are squashed into a new image with a single new layer"]
    pub fn set_squash(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.squash = Some(v.into());
        self
    }

    #[doc= "Set the field `suppress_output`.\nSuppress the build output and print image ID on success"]
    pub fn set_suppress_output(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.suppress_output = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\nName and optionally a tag in the 'name:tag' format"]
    pub fn set_tag(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tag = Some(v.into());
        self
    }

    #[doc= "Set the field `target`.\nSet the target build stage to build"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\nVersion of the underlying builder to use"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }

    #[doc= "Set the field `auth_config`.\n"]
    pub fn set_auth_config(mut self, v: impl Into<BlockAssignable<ImageBuildElAuthConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auth_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ulimit`.\n"]
    pub fn set_ulimit(mut self, v: impl Into<BlockAssignable<ImageBuildElUlimitEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ulimit = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ulimit = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ImageBuildEl {
    type O = BlockAssignable<ImageBuildEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImageBuildEl {
    #[doc= "Value to specify the build context. Currently, only a `PATH` context is supported. You can use the helper function '${path.cwd}/context-dir'. Please see https://docs.docker.com/build/building/context/ for more information about build contexts."]
    pub context: PrimField<String>,
}

impl BuildImageBuildEl {
    pub fn build(self) -> ImageBuildEl {
        ImageBuildEl {
            build_arg: core::default::Default::default(),
            build_args: core::default::Default::default(),
            build_id: core::default::Default::default(),
            cache_from: core::default::Default::default(),
            cgroup_parent: core::default::Default::default(),
            context: self.context,
            cpu_period: core::default::Default::default(),
            cpu_quota: core::default::Default::default(),
            cpu_set_cpus: core::default::Default::default(),
            cpu_set_mems: core::default::Default::default(),
            cpu_shares: core::default::Default::default(),
            dockerfile: core::default::Default::default(),
            extra_hosts: core::default::Default::default(),
            force_remove: core::default::Default::default(),
            isolation: core::default::Default::default(),
            label: core::default::Default::default(),
            labels: core::default::Default::default(),
            memory: core::default::Default::default(),
            memory_swap: core::default::Default::default(),
            network_mode: core::default::Default::default(),
            no_cache: core::default::Default::default(),
            platform: core::default::Default::default(),
            pull_parent: core::default::Default::default(),
            remote_context: core::default::Default::default(),
            remove: core::default::Default::default(),
            security_opt: core::default::Default::default(),
            session_id: core::default::Default::default(),
            shm_size: core::default::Default::default(),
            squash: core::default::Default::default(),
            suppress_output: core::default::Default::default(),
            tag: core::default::Default::default(),
            target: core::default::Default::default(),
            version: core::default::Default::default(),
            auth_config: core::default::Default::default(),
            ulimit: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ImageBuildElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImageBuildElRef {
    fn new(shared: StackShared, base: String) -> ImageBuildElRef {
        ImageBuildElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImageBuildElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_arg` after provisioning.\nSet build-time variables"]
    pub fn build_arg(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.build_arg", self.base))
    }

    #[doc= "Get a reference to the value of field `build_args` after provisioning.\nPairs for build-time variables in the form TODO"]
    pub fn build_args(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.build_args", self.base))
    }

    #[doc= "Get a reference to the value of field `build_id` after provisioning.\nBuildID is an optional identifier that can be passed together with the build request. The same identifier can be used to gracefully cancel the build with the cancel request."]
    pub fn build_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_id", self.base))
    }

    #[doc= "Get a reference to the value of field `cache_from` after provisioning.\nImages to consider as cache sources"]
    pub fn cache_from(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cache_from", self.base))
    }

    #[doc= "Get a reference to the value of field `cgroup_parent` after provisioning.\nOptional parent cgroup for the container"]
    pub fn cgroup_parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroup_parent", self.base))
    }

    #[doc= "Get a reference to the value of field `context` after provisioning.\nValue to specify the build context. Currently, only a `PATH` context is supported. You can use the helper function '${path.cwd}/context-dir'. Please see https://docs.docker.com/build/building/context/ for more information about build contexts."]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_period` after provisioning.\nThe length of a CPU period in microseconds"]
    pub fn cpu_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_period", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_quota` after provisioning.\nMicroseconds of CPU time that the container can get in a CPU period"]
    pub fn cpu_quota(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_quota", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_set_cpus` after provisioning.\nCPUs in which to allow execution (e.g., `0-3`, `0`, `1`)"]
    pub fn cpu_set_cpus(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_set_cpus", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_set_mems` after provisioning.\nMEMs in which to allow execution (`0-3`, `0`, `1`)"]
    pub fn cpu_set_mems(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_set_mems", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_shares` after provisioning.\nCPU shares (relative weight)"]
    pub fn cpu_shares(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_shares", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile` after provisioning.\nName of the Dockerfile. Defaults to `Dockerfile`."]
    pub fn dockerfile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile", self.base))
    }

    #[doc= "Get a reference to the value of field `extra_hosts` after provisioning.\nA list of hostnames/IP mappings to add to the container’s /etc/hosts file. Specified in the form [\"hostname:IP\"]"]
    pub fn extra_hosts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.extra_hosts", self.base))
    }

    #[doc= "Get a reference to the value of field `force_remove` after provisioning.\nAlways remove intermediate containers"]
    pub fn force_remove(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `isolation` after provisioning.\nIsolation represents the isolation technology of a container. The supported values are "]
    pub fn isolation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.isolation", self.base))
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\nSet metadata for an image"]
    pub fn label(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nUser-defined key/value metadata"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\nSet memory limit for build"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc= "Get a reference to the value of field `memory_swap` after provisioning.\nTotal memory (memory + swap), -1 to enable unlimited swap"]
    pub fn memory_swap(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.base))
    }

    #[doc= "Get a reference to the value of field `network_mode` after provisioning.\nSet the networking mode for the RUN instructions during build"]
    pub fn network_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `no_cache` after provisioning.\nDo not use the cache when building the image"]
    pub fn no_cache(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_cache", self.base))
    }

    #[doc= "Get a reference to the value of field `platform` after provisioning.\nSet platform if server is multi-platform capable"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }

    #[doc= "Get a reference to the value of field `pull_parent` after provisioning.\nAttempt to pull the image even if an older image exists locally"]
    pub fn pull_parent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pull_parent", self.base))
    }

    #[doc= "Get a reference to the value of field `remote_context` after provisioning.\nA Git repository URI or HTTP/HTTPS context URI"]
    pub fn remote_context(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_context", self.base))
    }

    #[doc= "Get a reference to the value of field `remove` after provisioning.\nRemove intermediate containers after a successful build. Defaults to `true`."]
    pub fn remove(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove", self.base))
    }

    #[doc= "Get a reference to the value of field `security_opt` after provisioning.\nThe security options"]
    pub fn security_opt(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.security_opt", self.base))
    }

    #[doc= "Get a reference to the value of field `session_id` after provisioning.\nSet an ID for the build session"]
    pub fn session_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_id", self.base))
    }

    #[doc= "Get a reference to the value of field `shm_size` after provisioning.\nSize of /dev/shm in bytes. The size must be greater than 0"]
    pub fn shm_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shm_size", self.base))
    }

    #[doc= "Get a reference to the value of field `squash` after provisioning.\nIf true the new layers are squashed into a new image with a single new layer"]
    pub fn squash(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash", self.base))
    }

    #[doc= "Get a reference to the value of field `suppress_output` after provisioning.\nSuppress the build output and print image ID on success"]
    pub fn suppress_output(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suppress_output", self.base))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nName and optionally a tag in the 'name:tag' format"]
    pub fn tag(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nSet the target build stage to build"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nVersion of the underlying builder to use"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc= "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> ListRef<ImageBuildElAuthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auth_config", self.base))
    }

    #[doc= "Get a reference to the value of field `ulimit` after provisioning.\n"]
    pub fn ulimit(&self) -> ListRef<ImageBuildElUlimitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ulimit", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImageDynamic {
    build: Option<DynamicBlock<ImageBuildEl>>,
}
