#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Serialize)]
struct ProviderDockerData_ {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_material: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_material: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_docker_daemon_check: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_material: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_opts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

struct ProviderDocker_ {
    data: std::cell::RefCell<ProviderDockerData_>,
}

pub struct ProviderDocker(std::rc::Rc<ProviderDocker_>);

impl ProviderDocker {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "docker", alias)
        } else {
            "docker".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc = "Set the field `ca_material`.\nPEM-encoded content of Docker host CA certificate"]
    pub fn set_ca_material(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ca_material = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cert_material`.\nPEM-encoded content of Docker client certificate"]
    pub fn set_cert_material(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cert_material = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cert_path`.\nPath to directory with Docker TLS config"]
    pub fn set_cert_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cert_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `context`.\nThe name of the Docker context to use. Can also be set via `DOCKER_CONTEXT` environment variable. Overrides the `host` if set."]
    pub fn set_context(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().context = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_docker_daemon_check`.\nIf set to `true`, the provider will not check if the Docker daemon is running. This is useful for resources/data_sourcess that do not require a running Docker daemon, such as the data source `docker_registry_image`."]
    pub fn set_disable_docker_daemon_check(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_docker_daemon_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe Docker daemon address"]
    pub fn set_host(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_material`.\nPEM-encoded content of Docker client private key"]
    pub fn set_key_material(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key_material = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_opts`.\nAdditional SSH option flags to be appended when using `ssh://` protocol"]
    pub fn set_ssh_opts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ssh_opts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderDocker_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "docker".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "kreuzwerker/docker",
            "version": "4.6.0",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderDocker {}

impl BuildProviderDocker {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderDocker {
        let out =
            ProviderDocker(std::rc::Rc::new(ProviderDocker_ { data: std::cell::RefCell::new(ProviderDockerData_ {
                alias: std::option::Option::None,
                ca_material: core::default::Default::default(),
                cert_material: core::default::Default::default(),
                cert_path: core::default::Default::default(),
                context: core::default::Default::default(),
                disable_docker_daemon_check: core::default::Default::default(),
                host: core::default::Default::default(),
                key_material: core::default::Default::default(),
                ssh_opts: core::default::Default::default(),
            }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
