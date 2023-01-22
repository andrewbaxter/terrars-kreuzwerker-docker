use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderDockerData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_material: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_material: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_material: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_opts: Option<ListField<PrimField<String>>>,
}

struct ProviderDocker_ {
    data: RefCell<ProviderDockerData>,
}

pub struct ProviderDocker(Rc<ProviderDocker_>);

impl ProviderDocker {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "docker", alias)
        } else {
            "docker".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `ca_material`.\nPEM-encoded content of Docker host CA certificate"]
    pub fn set_ca_material(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ca_material = Some(v.into());
        self
    }

    #[doc= "Set the field `cert_material`.\nPEM-encoded content of Docker client certificate"]
    pub fn set_cert_material(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cert_material = Some(v.into());
        self
    }

    #[doc= "Set the field `cert_path`.\nPath to directory with Docker TLS config"]
    pub fn set_cert_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cert_path = Some(v.into());
        self
    }

    #[doc= "Set the field `host`.\nThe Docker daemon address"]
    pub fn set_host(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().host = Some(v.into());
        self
    }

    #[doc= "Set the field `key_material`.\nPEM-encoded content of Docker client private key"]
    pub fn set_key_material(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_material = Some(v.into());
        self
    }

    #[doc= "Set the field `ssh_opts`.\nAdditional SSH option flags to be appended when using `ssh://` protocol"]
    pub fn set_ssh_opts(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ssh_opts = Some(v.into());
        self
    }
}

impl Provider for ProviderDocker_ {
    fn extract_type_tf_id(&self) -> String {
        "docker".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "kreuzwerker/docker",
            "version": "3.0.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderDocker {}

impl BuildProviderDocker {
    pub fn build(self, stack: &mut Stack) -> ProviderDocker {
        let out = ProviderDocker(Rc::new(ProviderDocker_ { data: RefCell::new(ProviderDockerData {
            alias: None,
            ca_material: core::default::Default::default(),
            cert_material: core::default::Default::default(),
            cert_path: core::default::Default::default(),
            host: core::default::Default::default(),
            key_material: core::default::Default::default(),
            ssh_opts: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
