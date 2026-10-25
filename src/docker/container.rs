#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct ContainerData_ {
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
    attach: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroup_parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cgroupns_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_read_refresh_timeout_milliseconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_quota: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_shares: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destroy_grace_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_cgroup_rules: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_opts: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_search: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domainname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_add: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipc_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_opts: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retry_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_reservation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_swap: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    must_run: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileged: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_all_ports: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove_volumes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restart: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rm: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_opts: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shm_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stdin_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_signal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_opts: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sysctls: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tmpfs: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tty: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    userns_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std::option::Option<std::vec::Vec<Container_Capabilities>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_read_bps: std::option::Option<std::vec::Vec<Container_DeviceReadBps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_read_iops: std::option::Option<std::vec::Vec<Container_DeviceReadIops>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_requests: std::option::Option<std::vec::Vec<Container_DeviceRequests>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_write_bps: std::option::Option<std::vec::Vec<Container_DeviceWriteBps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_write_iops: std::option::Option<std::vec::Vec<Container_DeviceWriteIops>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    devices: std::option::Option<std::vec::Vec<Container_Devices>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthcheck: std::option::Option<std::vec::Vec<Container_Healthcheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<std::vec::Vec<Container_Host>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<Container_Labels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mounts: std::option::Option<std::vec::Vec<Container_Mounts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks_advanced: std::option::Option<std::vec::Vec<Container_NetworksAdvanced>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<Container_Ports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Container_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ulimit: std::option::Option<std::vec::Vec<Container_Ulimit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload: std::option::Option<std::vec::Vec<Container_Upload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<Container_Volumes>>,
    dynamic: ContainerDynamic,
}

struct Container_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerData_>,
}

#[derive(Clone)]
pub struct Container(std::rc::Rc<Container_>);

impl Container {
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

    #[doc =
        "Set the field `attach`.\nIf `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`."]
    pub fn set_attach(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().attach = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cgroup_parent`.\nOptional parent cgroup for the container"]
    pub fn set_cgroup_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cgroup_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cgroupns_mode`.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn set_cgroupns_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cgroupns_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `command`.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.conf\"]`."]
    pub fn set_command(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().command = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `container_read_refresh_timeout_milliseconds`.\nThe total number of milliseconds to wait for the container to reach status 'running'"]
    pub fn set_container_read_refresh_timeout_milliseconds(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().container_read_refresh_timeout_milliseconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cpu_period`.\nSpecify the CPU CFS scheduler period (in microseconds), which is used alongside `cpu-quota`. Is ignored if `cpus` is set."]
    pub fn set_cpu_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpu_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cpu_quota`.\nImpose a CPU CFS quota on the container (in microseconds). The number of microseconds per `cpu-period` that the container is limited to before throttled. Is ignored if `cpus` is set."]
    pub fn set_cpu_quota(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpu_quota = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cpu_set`.\nA comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`."]
    pub fn set_cpu_set(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cpu_set = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_shares`.\nCPU shares (relative weight) for the container."]
    pub fn set_cpu_shares(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpu_shares = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cpus`.\nSpecify how much of the available CPU resources a container can use. e.g a value of 1.5 means the container is guaranteed at most one and a half of the CPUs. Has precedence over `cpu_period` and `cpu_quota`."]
    pub fn set_cpus(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `destroy_grace_seconds`.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn set_destroy_grace_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().destroy_grace_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `device_cgroup_rules`.\nCgroup rules to allow access to classes of devices without binding specific device nodes."]
    pub fn set_device_cgroup_rules(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().device_cgroup_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns`.\nDNS servers to use."]
    pub fn set_dns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_opts`.\nDNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options."]
    pub fn set_dns_opts(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dns_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_search`.\nDNS search domains that are used when bare unqualified hostnames are used inside of the container."]
    pub fn set_dns_search(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dns_search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domainname`.\nDomain name of the container."]
    pub fn set_domainname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domainname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `entrypoint`.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogram\"]`."]
    pub fn set_entrypoint(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\nEnvironment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn set_env(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().env = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gpus`.\nGPU devices to add to the container. Supported values are `all` or `device=<id[,id...]>`, for example `device=0,2` or `device=GPU-3a23c669-1f69-c64e-cf85-44e9b07e7a2a`."]
    pub fn set_gpus(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_add`.\nAdditional groups for the container user"]
    pub fn set_group_add(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().group_add = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nHostname of the container."]
    pub fn set_hostname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `init`.\nConfigured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults."]
    pub fn set_init(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().init = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipc_mode`.\nIPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`."]
    pub fn set_ipc_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipc_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_driver`.\nThe logging driver to use for the container."]
    pub fn set_log_driver(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_driver = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_opts`.\nKey/value pairs to use as options for the logging driver."]
    pub fn set_log_opts(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().log_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logs`.\nSave the container logs (`attach` must be enabled). Defaults to `false`."]
    pub fn set_logs(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_retry_count`.\nThe maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'."]
    pub fn set_max_retry_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_retry_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nThe memory limit for the container in MBs."]
    pub fn set_memory(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `memory_reservation`.\nThe memory-resveration for the container in MBs. Defaults to 0. Allows you to specify a soft limit smaller than `memory` which is activated when Docker detects contention or low memory on the host machine. If you use `memory-reservation`, it must be set lower than `memory` for it to take precedence. Because it is a soft limit, it doesn't guarantee that the container doesn't exceed the limit."]
    pub fn set_memory_reservation(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory_reservation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `memory_swap`.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn set_memory_swap(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory_swap = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `must_run`.\nIf `true`, then the Docker container will be kept running. If `false`, Terraform leaves the container alone. This attribute is also used to trigger a restart of a stopped container. If your container is stopped, Terraform will set `must_run` to `false` and this will trigger a change. Defaults to `true`."]
    pub fn set_must_run(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().must_run = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_mode`.\nNetwork mode of the container. Defaults to `bridge`. If your host OS is any other OS, you need to set this value explicitly, e.g. `nat` when your container will be running on an Windows host. See https://docs.docker.com/engine/network/ for more information."]
    pub fn set_network_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pid_mode`.\nThe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn set_pid_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pid_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `platform`.\nPlatform in the format `os[/arch[/variant]]` used for image lookup and container runtime, for example `linux/amd64`."]
    pub fn set_platform(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `privileged`.\nIf `true`, the container runs in privileged mode."]
    pub fn set_privileged(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().privileged = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publish_all_ports`.\nPublish all ports of the container."]
    pub fn set_publish_all_ports(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publish_all_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\nIf `true`, the container will be started as readonly. Defaults to `false`."]
    pub fn set_read_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remove_volumes`.\nIf `true`, it will remove anonymous volumes associated with the container. Defaults to `true`."]
    pub fn set_remove_volumes(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_volumes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restart`.\nThe restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`."]
    pub fn set_restart(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().restart = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rm`.\nIf `true`, then the container will be automatically removed when it exits. Defaults to `false`."]
    pub fn set_rm(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().rm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime`.\nRuntime to use for the container."]
    pub fn set_runtime(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().runtime = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_opts`.\nList of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration."]
    pub fn set_security_opts(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shm_size`.\nSize of `/dev/shm` in MBs."]
    pub fn set_shm_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().shm_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start`.\nIf `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`."]
    pub fn set_start(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stdin_open`.\nIf `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`."]
    pub fn set_stdin_open(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().stdin_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_signal`.\nSignal to stop a container (default `SIGTERM`)."]
    pub fn set_stop_signal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().stop_signal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_timeout`.\nTimeout (in seconds) to stop a container."]
    pub fn set_stop_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().stop_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_opts`.\nKey/value pairs for the storage driver options, e.g. `size`: `120G`"]
    pub fn set_storage_opts(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().storage_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sysctls`.\nA map of kernel parameters (sysctls) to set in the container."]
    pub fn set_sysctls(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().sysctls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tmpfs`.\nA map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options."]
    pub fn set_tmpfs(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tmpfs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tty`.\nIf `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`."]
    pub fn set_tty(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tty = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user`.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literally or by name."]
    pub fn set_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `userns_mode`.\nSets the usernamespace mode for the container when usernamespace remapping option is enabled."]
    pub fn set_userns_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().userns_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait`.\nIf `true`, then the Docker container is waited for being healthy state after creation. This requires your container to have a healthcheck, otherwise this provider will error. If `false`, then the container health state is not checked. Defaults to `false`."]
    pub fn set_wait(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_timeout`.\nThe timeout in seconds to wait the container to be healthy after creation. Defaults to `60`."]
    pub fn set_wait_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wait_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `working_dir`.\nThe working directory for commands to run in."]
    pub fn set_working_dir(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().working_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Capabilities>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capabilities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capabilities = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `device_read_bps`.\n"]
    pub fn set_device_read_bps(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_DeviceReadBps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_read_bps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_read_bps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `device_read_iops`.\n"]
    pub fn set_device_read_iops(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_DeviceReadIops>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_read_iops = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_read_iops = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `device_requests`.\n"]
    pub fn set_device_requests(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_DeviceRequests>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_requests = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_requests = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `device_write_bps`.\n"]
    pub fn set_device_write_bps(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_DeviceWriteBps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_write_bps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_write_bps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `device_write_iops`.\n"]
    pub fn set_device_write_iops(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_DeviceWriteIops>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_write_iops = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_write_iops = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `devices`.\n"]
    pub fn set_devices(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Devices>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().devices = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.devices = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `healthcheck`.\n"]
    pub fn set_healthcheck(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Healthcheck>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().healthcheck = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.healthcheck = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Host>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().host = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.host = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Labels>>) -> Self {
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

    #[doc = "Set the field `mounts`.\n"]
    pub fn set_mounts(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Mounts>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mounts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `networks_advanced`.\n"]
    pub fn set_networks_advanced(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_NetworksAdvanced>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks_advanced = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks_advanced = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Ports>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ports = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Container_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ulimit`.\n"]
    pub fn set_ulimit(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Ulimit>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ulimit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ulimit = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upload`.\n"]
    pub fn set_upload(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Upload>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Volumes>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.volumes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `attach` after provisioning.\nIf `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`."]
    pub fn attach(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attach", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bridge` after provisioning.\nThe network bridge of the container as read from its NetworkSettings."]
    pub fn bridge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bridge", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cgroup_parent` after provisioning.\nOptional parent cgroup for the container"]
    pub fn cgroup_parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroup_parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cgroupns_mode` after provisioning.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn cgroupns_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroupns_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.conf\"]`."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `container_logs` after provisioning.\nThe logs of the container if its execution is done (`attach` must be disabled)."]
    pub fn container_logs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_logs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `container_read_refresh_timeout_milliseconds` after provisioning.\nThe total number of milliseconds to wait for the container to reach status 'running'"]
    pub fn container_read_refresh_timeout_milliseconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_read_refresh_timeout_milliseconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cpu_period` after provisioning.\nSpecify the CPU CFS scheduler period (in microseconds), which is used alongside `cpu-quota`. Is ignored if `cpus` is set."]
    pub fn cpu_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_quota` after provisioning.\nImpose a CPU CFS quota on the container (in microseconds). The number of microseconds per `cpu-period` that the container is limited to before throttled. Is ignored if `cpus` is set."]
    pub fn cpu_quota(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_quota", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_set` after provisioning.\nA comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`."]
    pub fn cpu_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_shares` after provisioning.\nCPU shares (relative weight) for the container."]
    pub fn cpu_shares(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_shares", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpus` after provisioning.\nSpecify how much of the available CPU resources a container can use. e.g a value of 1.5 means the container is guaranteed at most one and a half of the CPUs. Has precedence over `cpu_period` and `cpu_quota`."]
    pub fn cpus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destroy_grace_seconds` after provisioning.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn destroy_grace_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destroy_grace_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_cgroup_rules` after provisioning.\nCgroup rules to allow access to classes of devices without binding specific device nodes."]
    pub fn device_cgroup_rules(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.device_cgroup_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\nDNS servers to use."]
    pub fn dns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_opts` after provisioning.\nDNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options."]
    pub fn dns_opts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_opts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_search` after provisioning.\nDNS search domains that are used when bare unqualified hostnames are used inside of the container."]
    pub fn dns_search(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_search", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domainname` after provisioning.\nDomain name of the container."]
    pub fn domainname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domainname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entrypoint` after provisioning.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogram\"]`."]
    pub fn entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nEnvironment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exit_code` after provisioning.\nThe exit code of the container if its execution is done (`must_run` must be disabled)."]
    pub fn exit_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exit_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gpus` after provisioning.\nGPU devices to add to the container. Supported values are `all` or `device=<id[,id...]>`, for example `device=0,2` or `device=GPU-3a23c669-1f69-c64e-cf85-44e9b07e7a2a`."]
    pub fn gpus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_add` after provisioning.\nAdditional groups for the container user"]
    pub fn group_add(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.group_add", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname of the container."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nThe ID of the image to back this container. The easiest way to get this value is to use the `image_id` attribute of the `docker_image` resource as is shown in the example."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `init` after provisioning.\nConfigured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults."]
    pub fn init(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.init", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipc_mode` after provisioning.\nIPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`."]
    pub fn ipc_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_driver` after provisioning.\nThe logging driver to use for the container."]
    pub fn log_driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_driver", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_opts` after provisioning.\nKey/value pairs to use as options for the logging driver."]
    pub fn log_opts(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.log_opts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logs` after provisioning.\nSave the container logs (`attach` must be enabled). Defaults to `false`."]
    pub fn logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_retry_count` after provisioning.\nThe maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'."]
    pub fn max_retry_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retry_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nThe memory limit for the container in MBs."]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_reservation` after provisioning.\nThe memory-resveration for the container in MBs. Defaults to 0. Allows you to specify a soft limit smaller than `memory` which is activated when Docker detects contention or low memory on the host machine. If you use `memory-reservation`, it must be set lower than `memory` for it to take precedence. Because it is a soft limit, it doesn't guarantee that the container doesn't exceed the limit."]
    pub fn memory_reservation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_reservation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_swap` after provisioning.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn memory_swap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `must_run` after provisioning.\nIf `true`, then the Docker container will be kept running. If `false`, Terraform leaves the container alone. This attribute is also used to trigger a restart of a stopped container. If your container is stopped, Terraform will set `must_run` to `false` and this will trigger a change. Defaults to `true`."]
    pub fn must_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.must_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the container."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_data` after provisioning.\nThe data of the networks the container is connected to."]
    pub fn network_data(&self) -> terrars::ListRef<Container_NetworkDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_mode` after provisioning.\nNetwork mode of the container. Defaults to `bridge`. If your host OS is any other OS, you need to set this value explicitly, e.g. `nat` when your container will be running on an Windows host. See https://docs.docker.com/engine/network/ for more information."]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pid_mode` after provisioning.\nThe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn pid_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `platform` after provisioning.\nPlatform in the format `os[/arch[/variant]]` used for image lookup and container runtime, for example `linux/amd64`."]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `privileged` after provisioning.\nIf `true`, the container runs in privileged mode."]
    pub fn privileged(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `publish_all_ports` after provisioning.\nPublish all ports of the container."]
    pub fn publish_all_ports(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_all_ports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIf `true`, the container will be started as readonly. Defaults to `false`."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_volumes` after provisioning.\nIf `true`, it will remove anonymous volumes associated with the container. Defaults to `true`."]
    pub fn remove_volumes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove_volumes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restart` after provisioning.\nThe restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`."]
    pub fn restart(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rm` after provisioning.\nIf `true`, then the container will be automatically removed when it exits. Defaults to `false`."]
    pub fn rm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nRuntime to use for the container."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_opts` after provisioning.\nList of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration."]
    pub fn security_opts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_opts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shm_size` after provisioning.\nSize of `/dev/shm` in MBs."]
    pub fn shm_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shm_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start` after provisioning.\nIf `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`."]
    pub fn start(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stdin_open` after provisioning.\nIf `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`."]
    pub fn stdin_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stdin_open", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to stop a container (default `SIGTERM`)."]
    pub fn stop_signal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stop_timeout` after provisioning.\nTimeout (in seconds) to stop a container."]
    pub fn stop_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_opts` after provisioning.\nKey/value pairs for the storage driver options, e.g. `size`: `120G`"]
    pub fn storage_opts(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.storage_opts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sysctls` after provisioning.\nA map of kernel parameters (sysctls) to set in the container."]
    pub fn sysctls(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.sysctls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tmpfs` after provisioning.\nA map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options."]
    pub fn tmpfs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tmpfs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tty` after provisioning.\nIf `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`."]
    pub fn tty(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tty", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literally or by name."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `userns_mode` after provisioning.\nSets the usernamespace mode for the container when usernamespace remapping option is enabled."]
    pub fn userns_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.userns_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nIf `true`, then the Docker container is waited for being healthy state after creation. This requires your container to have a healthcheck, otherwise this provider will error. If `false`, then the container health state is not checked. Defaults to `false`."]
    pub fn wait(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_timeout` after provisioning.\nThe timeout in seconds to wait the container to be healthy after creation. Defaults to `60`."]
    pub fn wait_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `working_dir` after provisioning.\nThe working directory for commands to run in."]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> terrars::ListRef<Container_HealthcheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<Container_PortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Container_TimeoutsRef {
        Container_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Container {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Container { }

impl terrars::ToListMappable for Container {
    type O = terrars::ListRef<ContainerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Container_ {
    fn extract_resource_type(&self) -> std::string::String {
        "docker_container".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainer {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the image to back this container. The easiest way to get this value is to use the `image_id` attribute of the `docker_image` resource as is shown in the example."]
    pub image: terrars::PrimField<std::string::String>,
    #[doc = "The name of the container."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildContainer {
    pub fn build(self, stack: &mut terrars::Stack) -> Container {
        let out = Container(std::rc::Rc::new(Container_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                attach: core::default::Default::default(),
                cgroup_parent: core::default::Default::default(),
                cgroupns_mode: core::default::Default::default(),
                command: core::default::Default::default(),
                container_read_refresh_timeout_milliseconds: core::default::Default::default(),
                cpu_period: core::default::Default::default(),
                cpu_quota: core::default::Default::default(),
                cpu_set: core::default::Default::default(),
                cpu_shares: core::default::Default::default(),
                cpus: core::default::Default::default(),
                destroy_grace_seconds: core::default::Default::default(),
                device_cgroup_rules: core::default::Default::default(),
                dns: core::default::Default::default(),
                dns_opts: core::default::Default::default(),
                dns_search: core::default::Default::default(),
                domainname: core::default::Default::default(),
                entrypoint: core::default::Default::default(),
                env: core::default::Default::default(),
                gpus: core::default::Default::default(),
                group_add: core::default::Default::default(),
                hostname: core::default::Default::default(),
                id: core::default::Default::default(),
                image: self.image,
                init: core::default::Default::default(),
                ipc_mode: core::default::Default::default(),
                log_driver: core::default::Default::default(),
                log_opts: core::default::Default::default(),
                logs: core::default::Default::default(),
                max_retry_count: core::default::Default::default(),
                memory: core::default::Default::default(),
                memory_reservation: core::default::Default::default(),
                memory_swap: core::default::Default::default(),
                must_run: core::default::Default::default(),
                name: self.name,
                network_mode: core::default::Default::default(),
                pid_mode: core::default::Default::default(),
                platform: core::default::Default::default(),
                privileged: core::default::Default::default(),
                publish_all_ports: core::default::Default::default(),
                read_only: core::default::Default::default(),
                remove_volumes: core::default::Default::default(),
                restart: core::default::Default::default(),
                rm: core::default::Default::default(),
                runtime: core::default::Default::default(),
                security_opts: core::default::Default::default(),
                shm_size: core::default::Default::default(),
                start: core::default::Default::default(),
                stdin_open: core::default::Default::default(),
                stop_signal: core::default::Default::default(),
                stop_timeout: core::default::Default::default(),
                storage_opts: core::default::Default::default(),
                sysctls: core::default::Default::default(),
                tmpfs: core::default::Default::default(),
                tty: core::default::Default::default(),
                user: core::default::Default::default(),
                userns_mode: core::default::Default::default(),
                wait: core::default::Default::default(),
                wait_timeout: core::default::Default::default(),
                working_dir: core::default::Default::default(),
                capabilities: core::default::Default::default(),
                device_read_bps: core::default::Default::default(),
                device_read_iops: core::default::Default::default(),
                device_requests: core::default::Default::default(),
                device_write_bps: core::default::Default::default(),
                device_write_iops: core::default::Default::default(),
                devices: core::default::Default::default(),
                healthcheck: core::default::Default::default(),
                host: core::default::Default::default(),
                labels: core::default::Default::default(),
                mounts: core::default::Default::default(),
                networks_advanced: core::default::Default::default(),
                ports: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ulimit: core::default::Default::default(),
                upload: core::default::Default::default(),
                volumes: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attach` after provisioning.\nIf `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`."]
    pub fn attach(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attach", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bridge` after provisioning.\nThe network bridge of the container as read from its NetworkSettings."]
    pub fn bridge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bridge", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cgroup_parent` after provisioning.\nOptional parent cgroup for the container"]
    pub fn cgroup_parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroup_parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cgroupns_mode` after provisioning.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn cgroupns_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroupns_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.conf\"]`."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `container_logs` after provisioning.\nThe logs of the container if its execution is done (`attach` must be disabled)."]
    pub fn container_logs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_logs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `container_read_refresh_timeout_milliseconds` after provisioning.\nThe total number of milliseconds to wait for the container to reach status 'running'"]
    pub fn container_read_refresh_timeout_milliseconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_read_refresh_timeout_milliseconds", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cpu_period` after provisioning.\nSpecify the CPU CFS scheduler period (in microseconds), which is used alongside `cpu-quota`. Is ignored if `cpus` is set."]
    pub fn cpu_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_quota` after provisioning.\nImpose a CPU CFS quota on the container (in microseconds). The number of microseconds per `cpu-period` that the container is limited to before throttled. Is ignored if `cpus` is set."]
    pub fn cpu_quota(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_quota", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_set` after provisioning.\nA comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`."]
    pub fn cpu_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_set", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_shares` after provisioning.\nCPU shares (relative weight) for the container."]
    pub fn cpu_shares(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_shares", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpus` after provisioning.\nSpecify how much of the available CPU resources a container can use. e.g a value of 1.5 means the container is guaranteed at most one and a half of the CPUs. Has precedence over `cpu_period` and `cpu_quota`."]
    pub fn cpus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destroy_grace_seconds` after provisioning.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn destroy_grace_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destroy_grace_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_cgroup_rules` after provisioning.\nCgroup rules to allow access to classes of devices without binding specific device nodes."]
    pub fn device_cgroup_rules(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.device_cgroup_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\nDNS servers to use."]
    pub fn dns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_opts` after provisioning.\nDNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options."]
    pub fn dns_opts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_opts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_search` after provisioning.\nDNS search domains that are used when bare unqualified hostnames are used inside of the container."]
    pub fn dns_search(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_search", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domainname` after provisioning.\nDomain name of the container."]
    pub fn domainname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domainname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entrypoint` after provisioning.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogram\"]`."]
    pub fn entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nEnvironment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exit_code` after provisioning.\nThe exit code of the container if its execution is done (`must_run` must be disabled)."]
    pub fn exit_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exit_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gpus` after provisioning.\nGPU devices to add to the container. Supported values are `all` or `device=<id[,id...]>`, for example `device=0,2` or `device=GPU-3a23c669-1f69-c64e-cf85-44e9b07e7a2a`."]
    pub fn gpus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_add` after provisioning.\nAdditional groups for the container user"]
    pub fn group_add(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.group_add", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname of the container."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nThe ID of the image to back this container. The easiest way to get this value is to use the `image_id` attribute of the `docker_image` resource as is shown in the example."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `init` after provisioning.\nConfigured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults."]
    pub fn init(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.init", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipc_mode` after provisioning.\nIPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`."]
    pub fn ipc_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_driver` after provisioning.\nThe logging driver to use for the container."]
    pub fn log_driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_driver", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_opts` after provisioning.\nKey/value pairs to use as options for the logging driver."]
    pub fn log_opts(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.log_opts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logs` after provisioning.\nSave the container logs (`attach` must be enabled). Defaults to `false`."]
    pub fn logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_retry_count` after provisioning.\nThe maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'."]
    pub fn max_retry_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retry_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nThe memory limit for the container in MBs."]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_reservation` after provisioning.\nThe memory-resveration for the container in MBs. Defaults to 0. Allows you to specify a soft limit smaller than `memory` which is activated when Docker detects contention or low memory on the host machine. If you use `memory-reservation`, it must be set lower than `memory` for it to take precedence. Because it is a soft limit, it doesn't guarantee that the container doesn't exceed the limit."]
    pub fn memory_reservation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_reservation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_swap` after provisioning.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn memory_swap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `must_run` after provisioning.\nIf `true`, then the Docker container will be kept running. If `false`, Terraform leaves the container alone. This attribute is also used to trigger a restart of a stopped container. If your container is stopped, Terraform will set `must_run` to `false` and this will trigger a change. Defaults to `true`."]
    pub fn must_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.must_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the container."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_data` after provisioning.\nThe data of the networks the container is connected to."]
    pub fn network_data(&self) -> terrars::ListRef<Container_NetworkDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_mode` after provisioning.\nNetwork mode of the container. Defaults to `bridge`. If your host OS is any other OS, you need to set this value explicitly, e.g. `nat` when your container will be running on an Windows host. See https://docs.docker.com/engine/network/ for more information."]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pid_mode` after provisioning.\nThe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn pid_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `platform` after provisioning.\nPlatform in the format `os[/arch[/variant]]` used for image lookup and container runtime, for example `linux/amd64`."]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `privileged` after provisioning.\nIf `true`, the container runs in privileged mode."]
    pub fn privileged(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `publish_all_ports` after provisioning.\nPublish all ports of the container."]
    pub fn publish_all_ports(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_all_ports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIf `true`, the container will be started as readonly. Defaults to `false`."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_volumes` after provisioning.\nIf `true`, it will remove anonymous volumes associated with the container. Defaults to `true`."]
    pub fn remove_volumes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove_volumes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restart` after provisioning.\nThe restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`."]
    pub fn restart(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restart", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rm` after provisioning.\nIf `true`, then the container will be automatically removed when it exits. Defaults to `false`."]
    pub fn rm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nRuntime to use for the container."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_opts` after provisioning.\nList of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration."]
    pub fn security_opts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_opts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shm_size` after provisioning.\nSize of `/dev/shm` in MBs."]
    pub fn shm_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shm_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start` after provisioning.\nIf `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`."]
    pub fn start(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stdin_open` after provisioning.\nIf `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`."]
    pub fn stdin_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stdin_open", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to stop a container (default `SIGTERM`)."]
    pub fn stop_signal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stop_timeout` after provisioning.\nTimeout (in seconds) to stop a container."]
    pub fn stop_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_opts` after provisioning.\nKey/value pairs for the storage driver options, e.g. `size`: `120G`"]
    pub fn storage_opts(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.storage_opts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sysctls` after provisioning.\nA map of kernel parameters (sysctls) to set in the container."]
    pub fn sysctls(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.sysctls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tmpfs` after provisioning.\nA map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options."]
    pub fn tmpfs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tmpfs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tty` after provisioning.\nIf `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`."]
    pub fn tty(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tty", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literally or by name."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `userns_mode` after provisioning.\nSets the usernamespace mode for the container when usernamespace remapping option is enabled."]
    pub fn userns_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.userns_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nIf `true`, then the Docker container is waited for being healthy state after creation. This requires your container to have a healthcheck, otherwise this provider will error. If `false`, then the container health state is not checked. Defaults to `false`."]
    pub fn wait(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_timeout` after provisioning.\nThe timeout in seconds to wait the container to be healthy after creation. Defaults to `60`."]
    pub fn wait_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `working_dir` after provisioning.\nThe working directory for commands to run in."]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> terrars::ListRef<Container_HealthcheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<Container_PortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Container_TimeoutsRef {
        Container_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ContainerDynamic {
    capabilities: std::option::Option<terrars::DynamicBlock<Container_Capabilities>>,
    device_read_bps: std::option::Option<terrars::DynamicBlock<Container_DeviceReadBps>>,
    device_read_iops: std::option::Option<terrars::DynamicBlock<Container_DeviceReadIops>>,
    device_requests: std::option::Option<terrars::DynamicBlock<Container_DeviceRequests>>,
    device_write_bps: std::option::Option<terrars::DynamicBlock<Container_DeviceWriteBps>>,
    device_write_iops: std::option::Option<terrars::DynamicBlock<Container_DeviceWriteIops>>,
    devices: std::option::Option<terrars::DynamicBlock<Container_Devices>>,
    healthcheck: std::option::Option<terrars::DynamicBlock<Container_Healthcheck>>,
    host: std::option::Option<terrars::DynamicBlock<Container_Host>>,
    labels: std::option::Option<terrars::DynamicBlock<Container_Labels>>,
    mounts: std::option::Option<terrars::DynamicBlock<Container_Mounts>>,
    networks_advanced: std::option::Option<terrars::DynamicBlock<Container_NetworksAdvanced>>,
    ports: std::option::Option<terrars::DynamicBlock<Container_Ports>>,
    ulimit: std::option::Option<terrars::DynamicBlock<Container_Ulimit>>,
    upload: std::option::Option<terrars::DynamicBlock<Container_Upload>>,
    volumes: std::option::Option<terrars::DynamicBlock<Container_Volumes>>,
}

#[derive(Serialize)]
pub struct Container_NetworkData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_ipv6_prefix_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_prefix_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_gateway: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mac_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Container_NetworkData {
    #[doc = "Set the field `gateway`.\n"]
    pub fn set_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `global_ipv6_address`.\n"]
    pub fn set_global_ipv6_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.global_ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `global_ipv6_prefix_length`.\n"]
    pub fn set_global_ipv6_prefix_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.global_ipv6_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_prefix_length`.\n"]
    pub fn set_ip_prefix_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ip_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_gateway`.\n"]
    pub fn set_ipv6_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mac_address`.\n"]
    pub fn set_mac_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mac_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_name`.\n"]
    pub fn set_network_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_NetworkData {
    type O = terrars::BlockAssignable<Container_NetworkData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_NetworkData {}

impl BuildContainer_NetworkData {
    pub fn build(self) -> Container_NetworkData {
        Container_NetworkData {
            gateway: core::default::Default::default(),
            global_ipv6_address: core::default::Default::default(),
            global_ipv6_prefix_length: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            ip_prefix_length: core::default::Default::default(),
            ipv6_gateway: core::default::Default::default(),
            mac_address: core::default::Default::default(),
            network_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_NetworkDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_NetworkDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_NetworkDataRef {
        Container_NetworkDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_NetworkDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `global_ipv6_address` after provisioning.\n"]
    pub fn global_ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_ipv6_address", self.base))
    }

    #[doc = "Get a reference to the value of field `global_ipv6_prefix_length` after provisioning.\n"]
    pub fn global_ipv6_prefix_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_ipv6_prefix_length", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_prefix_length` after provisioning.\n"]
    pub fn ip_prefix_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_prefix_length", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_gateway` after provisioning.\n"]
    pub fn ipv6_gateway(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mac_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network_name` after provisioning.\n"]
    pub fn network_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_name", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Capabilities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl Container_Capabilities {
    #[doc = "Set the field `add`.\nList of linux capabilities to add."]
    pub fn set_add(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.add = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drop`.\nList of linux capabilities to drop."]
    pub fn set_drop(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.drop = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Capabilities {
    type O = terrars::BlockAssignable<Container_Capabilities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Capabilities {}

impl BuildContainer_Capabilities {
    pub fn build(self) -> Container_Capabilities {
        Container_Capabilities {
            add: core::default::Default::default(),
            drop: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_CapabilitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_CapabilitiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_CapabilitiesRef {
        Container_CapabilitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_CapabilitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add` after provisioning.\nList of linux capabilities to add."]
    pub fn add(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.add", self.base))
    }

    #[doc = "Get a reference to the value of field `drop` after provisioning.\nList of linux capabilities to drop."]
    pub fn drop(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.drop", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_DeviceReadBps {
    path: terrars::PrimField<std::string::String>,
    rate: terrars::PrimField<f64>,
}

impl Container_DeviceReadBps { }

impl terrars::ToListMappable for Container_DeviceReadBps {
    type O = terrars::BlockAssignable<Container_DeviceReadBps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_DeviceReadBps {
    #[doc = "The device path on the host, e.g. `/dev/sda`."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The read rate limit in bytes per second."]
    pub rate: terrars::PrimField<f64>,
}

impl BuildContainer_DeviceReadBps {
    pub fn build(self) -> Container_DeviceReadBps {
        Container_DeviceReadBps {
            path: self.path,
            rate: self.rate,
        }
    }
}

#[allow(dead_code)]
pub struct Container_DeviceReadBpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_DeviceReadBpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_DeviceReadBpsRef {
        Container_DeviceReadBpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_DeviceReadBpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe device path on the host, e.g. `/dev/sda`."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rate` after provisioning.\nThe read rate limit in bytes per second."]
    pub fn rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_DeviceReadIops {
    path: terrars::PrimField<std::string::String>,
    rate: terrars::PrimField<f64>,
}

impl Container_DeviceReadIops { }

impl terrars::ToListMappable for Container_DeviceReadIops {
    type O = terrars::BlockAssignable<Container_DeviceReadIops>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_DeviceReadIops {
    #[doc = "The device path on the host, e.g. `/dev/sda`."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The read IOPS limit."]
    pub rate: terrars::PrimField<f64>,
}

impl BuildContainer_DeviceReadIops {
    pub fn build(self) -> Container_DeviceReadIops {
        Container_DeviceReadIops {
            path: self.path,
            rate: self.rate,
        }
    }
}

#[allow(dead_code)]
pub struct Container_DeviceReadIopsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_DeviceReadIopsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_DeviceReadIopsRef {
        Container_DeviceReadIopsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_DeviceReadIopsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe device path on the host, e.g. `/dev/sda`."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `rate` after provisioning.\nThe read IOPS limit."]
    pub fn rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_DeviceRequests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl Container_DeviceRequests {
    #[doc =
        "Set the field `capabilities`.\nList of device capabilities. Only used with `nvidia` driver (e.g., `gpu`, `compute`, `utility`)."]
    pub fn set_capabilities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.capabilities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `count`.\nNumber of devices to request. Use -1 for all devices. Only used with `nvidia` driver."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_ids`.\nList of device IDs or CDI device identifiers (e.g., `nvidia.com/gpu=all`)."]
    pub fn set_device_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.device_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `driver`.\nThe device driver to use. Common values: `cdi` for CDI devices, `nvidia` for NVIDIA GPU requests."]
    pub fn set_driver(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.driver = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `options`.\nDriver-specific options."]
    pub fn set_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_DeviceRequests {
    type O = terrars::BlockAssignable<Container_DeviceRequests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_DeviceRequests {}

impl BuildContainer_DeviceRequests {
    pub fn build(self) -> Container_DeviceRequests {
        Container_DeviceRequests {
            capabilities: core::default::Default::default(),
            count: core::default::Default::default(),
            device_ids: core::default::Default::default(),
            driver: core::default::Default::default(),
            options: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_DeviceRequestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_DeviceRequestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_DeviceRequestsRef {
        Container_DeviceRequestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_DeviceRequestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `capabilities` after provisioning.\nList of device capabilities. Only used with `nvidia` driver (e.g., `gpu`, `compute`, `utility`)."]
    pub fn capabilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nNumber of devices to request. Use -1 for all devices. Only used with `nvidia` driver."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `device_ids` after provisioning.\nList of device IDs or CDI device identifiers (e.g., `nvidia.com/gpu=all`)."]
    pub fn device_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.device_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `driver` after provisioning.\nThe device driver to use. Common values: `cdi` for CDI devices, `nvidia` for NVIDIA GPU requests."]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\nDriver-specific options."]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_DeviceWriteBps {
    path: terrars::PrimField<std::string::String>,
    rate: terrars::PrimField<f64>,
}

impl Container_DeviceWriteBps { }

impl terrars::ToListMappable for Container_DeviceWriteBps {
    type O = terrars::BlockAssignable<Container_DeviceWriteBps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_DeviceWriteBps {
    #[doc = "The device path on the host, e.g. `/dev/sda`."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The write rate limit in bytes per second."]
    pub rate: terrars::PrimField<f64>,
}

impl BuildContainer_DeviceWriteBps {
    pub fn build(self) -> Container_DeviceWriteBps {
        Container_DeviceWriteBps {
            path: self.path,
            rate: self.rate,
        }
    }
}

#[allow(dead_code)]
pub struct Container_DeviceWriteBpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_DeviceWriteBpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_DeviceWriteBpsRef {
        Container_DeviceWriteBpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_DeviceWriteBpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe device path on the host, e.g. `/dev/sda`."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rate` after provisioning.\nThe write rate limit in bytes per second."]
    pub fn rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_DeviceWriteIops {
    path: terrars::PrimField<std::string::String>,
    rate: terrars::PrimField<f64>,
}

impl Container_DeviceWriteIops { }

impl terrars::ToListMappable for Container_DeviceWriteIops {
    type O = terrars::BlockAssignable<Container_DeviceWriteIops>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_DeviceWriteIops {
    #[doc = "The device path on the host, e.g. `/dev/sda`."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "The write IOPS limit."]
    pub rate: terrars::PrimField<f64>,
}

impl BuildContainer_DeviceWriteIops {
    pub fn build(self) -> Container_DeviceWriteIops {
        Container_DeviceWriteIops {
            path: self.path,
            rate: self.rate,
        }
    }
}

#[allow(dead_code)]
pub struct Container_DeviceWriteIopsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_DeviceWriteIopsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_DeviceWriteIopsRef {
        Container_DeviceWriteIopsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_DeviceWriteIopsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe device path on the host, e.g. `/dev/sda`."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `rate` after provisioning.\nThe write IOPS limit."]
    pub fn rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Devices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    host_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Container_Devices {
    #[doc =
        "Set the field `container_path`.\nThe path in the container where the device will be bound. If not set, it defaults to the value of `host_path`."]
    pub fn set_container_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permissions`.\nThe cgroup permissions given to the container to access the device. Defaults to `rwm`."]
    pub fn set_permissions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Devices {
    type O = terrars::BlockAssignable<Container_Devices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Devices {
    #[doc = "The path on the host where the device is located."]
    pub host_path: terrars::PrimField<std::string::String>,
}

impl BuildContainer_Devices {
    pub fn build(self) -> Container_Devices {
        Container_Devices {
            container_path: core::default::Default::default(),
            host_path: self.host_path,
            permissions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_DevicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_DevicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_DevicesRef {
        Container_DevicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_DevicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_path` after provisioning.\nThe path in the container where the device will be bound. If not set, it defaults to the value of `host_path`."]
    pub fn container_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_path` after provisioning.\nThe path on the host where the device is located."]
    pub fn host_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nThe cgroup permissions given to the container to access the device. Defaults to `rwm`."]
    pub fn permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Healthcheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Container_Healthcheck {
    #[doc = "Set the field `interval`.\nTime between running the check (ms|s|m|h). Defaults to `0s`."]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retries`.\nConsecutive failures needed to report unhealthy. Defaults to `0`."]
    pub fn set_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_interval`.\nInterval before the healthcheck starts (ms|s|m|h). Defaults to `0s`."]
    pub fn set_start_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_period`.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn set_start_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `test`.\nCommand to run to check health. For example, to run `curl -f localhost/health` set the command to be `[\"CMD\", \"curl\", \"-f\", \"localhost/health\"]`. It works in the same way, and has the same default values, as the HEALTHCHECK Dockerfile instruction set by the service's Docker image. Your Compose file can override the values set in the Dockerfile."]
    pub fn set_test(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.test = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Healthcheck {
    type O = terrars::BlockAssignable<Container_Healthcheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Healthcheck {}

impl BuildContainer_Healthcheck {
    pub fn build(self) -> Container_Healthcheck {
        Container_Healthcheck {
            interval: core::default::Default::default(),
            retries: core::default::Default::default(),
            start_interval: core::default::Default::default(),
            start_period: core::default::Default::default(),
            test: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_HealthcheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_HealthcheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_HealthcheckRef {
        Container_HealthcheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_HealthcheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nTime between running the check (ms|s|m|h). Defaults to `0s`."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retries` after provisioning.\nConsecutive failures needed to report unhealthy. Defaults to `0`."]
    pub fn retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_interval` after provisioning.\nInterval before the healthcheck starts (ms|s|m|h). Defaults to `0s`."]
    pub fn start_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_period` after provisioning.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn start_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `test` after provisioning.\nCommand to run to check health. For example, to run `curl -f localhost/health` set the command to be `[\"CMD\", \"curl\", \"-f\", \"localhost/health\"]`. It works in the same way, and has the same default values, as the HEALTHCHECK Dockerfile instruction set by the service's Docker image. Your Compose file can override the values set in the Dockerfile."]
    pub fn test(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Host {
    host: terrars::PrimField<std::string::String>,
    ip: terrars::PrimField<std::string::String>,
}

impl Container_Host { }

impl terrars::ToListMappable for Container_Host {
    type O = terrars::BlockAssignable<Container_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Host {
    #[doc = "Hostname to add"]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "IP address this hostname should resolve to."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildContainer_Host {
    pub fn build(self) -> Container_Host {
        Container_Host {
            host: self.host,
            ip: self.ip,
        }
    }
}

#[allow(dead_code)]
pub struct Container_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_HostRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_HostRef {
        Container_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_HostRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nHostname to add"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nIP address this hostname should resolve to."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Labels {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Container_Labels { }

impl terrars::ToListMappable for Container_Labels {
    type O = terrars::BlockAssignable<Container_Labels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Labels {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildContainer_Labels {
    pub fn build(self) -> Container_Labels {
        Container_Labels {
            label: self.label,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Container_LabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_LabelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_LabelsRef {
        Container_LabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_LabelsRef {
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

#[derive(Serialize)]
pub struct Container_Mounts_BindOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Container_Mounts_BindOptions {
    #[doc = "Set the field `propagation`.\nA propagation mode with the value."]
    pub fn set_propagation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.propagation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Mounts_BindOptions {
    type O = terrars::BlockAssignable<Container_Mounts_BindOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Mounts_BindOptions {}

impl BuildContainer_Mounts_BindOptions {
    pub fn build(self) -> Container_Mounts_BindOptions {
        Container_Mounts_BindOptions { propagation: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Container_Mounts_BindOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_Mounts_BindOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_Mounts_BindOptionsRef {
        Container_Mounts_BindOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_Mounts_BindOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `propagation` after provisioning.\nA propagation mode with the value."]
    pub fn propagation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagation", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Mounts_TmpfsOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_bytes: std::option::Option<terrars::PrimField<f64>>,
}

impl Container_Mounts_TmpfsOptions {
    #[doc = "Set the field `mode`.\nThe permission mode for the tmpfs mount in an integer."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_bytes`.\nThe size for the tmpfs mount in bytes."]
    pub fn set_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Mounts_TmpfsOptions {
    type O = terrars::BlockAssignable<Container_Mounts_TmpfsOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Mounts_TmpfsOptions {}

impl BuildContainer_Mounts_TmpfsOptions {
    pub fn build(self) -> Container_Mounts_TmpfsOptions {
        Container_Mounts_TmpfsOptions {
            mode: core::default::Default::default(),
            size_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_Mounts_TmpfsOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_Mounts_TmpfsOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_Mounts_TmpfsOptionsRef {
        Container_Mounts_TmpfsOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_Mounts_TmpfsOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe permission mode for the tmpfs mount in an integer."]
    pub fn mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_bytes` after provisioning.\nThe size for the tmpfs mount in bytes."]
    pub fn size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_bytes", self.base))
    }
}

#[derive(Serialize, Default)]
struct Container_Mounts_VolumeOptionsDynamic {
    labels: std::option::Option<terrars::DynamicBlock<Container_Labels>>,
}

#[derive(Serialize)]
pub struct Container_Mounts_VolumeOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_copy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subpath: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<Container_Labels>>,
    dynamic: Container_Mounts_VolumeOptionsDynamic,
}

impl Container_Mounts_VolumeOptions {
    #[doc = "Set the field `driver_name`.\nName of the driver to use to create the volume."]
    pub fn set_driver_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.driver_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver_options`.\nkey/value map of driver specific options."]
    pub fn set_driver_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_copy`.\nPopulate volume with data from the target."]
    pub fn set_no_copy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_copy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subpath`.\nPath within the volume to mount. Requires docker server version 1.45 or higher."]
    pub fn set_subpath(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subpath = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl std::convert::Into<terrars::BlockAssignable<Container_Labels>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Container_Mounts_VolumeOptions {
    type O = terrars::BlockAssignable<Container_Mounts_VolumeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Mounts_VolumeOptions {}

impl BuildContainer_Mounts_VolumeOptions {
    pub fn build(self) -> Container_Mounts_VolumeOptions {
        Container_Mounts_VolumeOptions {
            driver_name: core::default::Default::default(),
            driver_options: core::default::Default::default(),
            no_copy: core::default::Default::default(),
            subpath: core::default::Default::default(),
            labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_Mounts_VolumeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_Mounts_VolumeOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_Mounts_VolumeOptionsRef {
        Container_Mounts_VolumeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_Mounts_VolumeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_name` after provisioning.\nName of the driver to use to create the volume."]
    pub fn driver_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `driver_options` after provisioning.\nkey/value map of driver specific options."]
    pub fn driver_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_copy` after provisioning.\nPopulate volume with data from the target."]
    pub fn no_copy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_copy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subpath` after provisioning.\nPath within the volume to mount. Requires docker server version 1.45 or higher."]
    pub fn subpath(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subpath", self.base))
    }
}

#[derive(Serialize, Default)]
struct Container_MountsDynamic {
    bind_options: std::option::Option<terrars::DynamicBlock<Container_Mounts_BindOptions>>,
    tmpfs_options: std::option::Option<terrars::DynamicBlock<Container_Mounts_TmpfsOptions>>,
    volume_options: std::option::Option<terrars::DynamicBlock<Container_Mounts_VolumeOptions>>,
}

#[derive(Serialize)]
pub struct Container_Mounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bind_options: std::option::Option<std::vec::Vec<Container_Mounts_BindOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tmpfs_options: std::option::Option<std::vec::Vec<Container_Mounts_TmpfsOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_options: std::option::Option<std::vec::Vec<Container_Mounts_VolumeOptions>>,
    dynamic: Container_MountsDynamic,
}

impl Container_Mounts {
    #[doc = "Set the field `read_only`.\nWhether the mount should be read-only."]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\nMount source (e.g. a volume name, a host path)."]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bind_options`.\n"]
    pub fn set_bind_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_Mounts_BindOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bind_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bind_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tmpfs_options`.\n"]
    pub fn set_tmpfs_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_Mounts_TmpfsOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tmpfs_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tmpfs_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volume_options`.\n"]
    pub fn set_volume_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Container_Mounts_VolumeOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volume_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Container_Mounts {
    type O = terrars::BlockAssignable<Container_Mounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Mounts {
    #[doc = "Container path"]
    pub target: terrars::PrimField<std::string::String>,
    #[doc = "The mount type"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainer_Mounts {
    pub fn build(self) -> Container_Mounts {
        Container_Mounts {
            read_only: core::default::Default::default(),
            source: core::default::Default::default(),
            target: self.target,
            type_: self.type_,
            bind_options: core::default::Default::default(),
            tmpfs_options: core::default::Default::default(),
            volume_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_MountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_MountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_MountsRef {
        Container_MountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_MountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nWhether the mount should be read-only."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nMount source (e.g. a volume name, a host path)."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\nContainer path"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe mount type"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `bind_options` after provisioning.\n"]
    pub fn bind_options(&self) -> terrars::ListRef<Container_Mounts_BindOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bind_options", self.base))
    }

    #[doc = "Get a reference to the value of field `tmpfs_options` after provisioning.\n"]
    pub fn tmpfs_options(&self) -> terrars::ListRef<Container_Mounts_TmpfsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tmpfs_options", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_options` after provisioning.\n"]
    pub fn volume_options(&self) -> terrars::ListRef<Container_Mounts_VolumeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_options", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_NetworksAdvanced {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aliases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_opts: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gw_priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    link_local_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mac_address: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl Container_NetworksAdvanced {
    #[doc = "Set the field `aliases`.\nThe network aliases of the container in the specific network."]
    pub fn set_aliases(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aliases = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `driver_opts`.\nAn array of driver options for the network endpoint, e.g. `opts1=value`. This is the equivalent to repeating `--driver-opt` for `docker run`."]
    pub fn set_driver_opts(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gw_priority`.\nGateway priority for this endpoint. The endpoint with the highest priority will provide the default gateway for the container. This is the equivalent to `--gw-priority` for `docker run`."]
    pub fn set_gw_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gw_priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_address`.\nThe IPV4 address of the container in the specific network."]
    pub fn set_ipv4_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address`.\nThe IPV6 address of the container in the specific network."]
    pub fn set_ipv6_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `link_local_ips`.\nThe link-local IPs of the container in the specific network. This is the equivalent to repeating `--link-local-ip` for `docker run`."]
    pub fn set_link_local_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.link_local_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mac_address`.\nThe MAC address of the container in the specific network."]
    pub fn set_mac_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mac_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_NetworksAdvanced {
    type O = terrars::BlockAssignable<Container_NetworksAdvanced>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_NetworksAdvanced {
    #[doc =
        "The name or id of the network to use. You can use `name` or `id` attribute from a `docker_network` resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildContainer_NetworksAdvanced {
    pub fn build(self) -> Container_NetworksAdvanced {
        Container_NetworksAdvanced {
            aliases: core::default::Default::default(),
            driver_opts: core::default::Default::default(),
            gw_priority: core::default::Default::default(),
            ipv4_address: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            link_local_ips: core::default::Default::default(),
            mac_address: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct Container_NetworksAdvancedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_NetworksAdvancedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_NetworksAdvancedRef {
        Container_NetworksAdvancedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_NetworksAdvancedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aliases` after provisioning.\nThe network aliases of the container in the specific network."]
    pub fn aliases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aliases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `driver_opts` after provisioning.\nAn array of driver options for the network endpoint, e.g. `opts1=value`. This is the equivalent to repeating `--driver-opt` for `docker run`."]
    pub fn driver_opts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.driver_opts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gw_priority` after provisioning.\nGateway priority for this endpoint. The endpoint with the highest priority will provide the default gateway for the container. This is the equivalent to `--gw-priority` for `docker run`."]
    pub fn gw_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gw_priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_address` after provisioning.\nThe IPV4 address of the container in the specific network."]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_address` after provisioning.\nThe IPV6 address of the container in the specific network."]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `link_local_ips` after provisioning.\nThe link-local IPs of the container in the specific network. This is the equivalent to repeating `--link-local-ip` for `docker run`."]
    pub fn link_local_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.link_local_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mac_address` after provisioning.\nThe MAC address of the container in the specific network."]
    pub fn mac_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mac_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name or id of the network to use. You can use `name` or `id` attribute from a `docker_network` resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Ports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external: std::option::Option<terrars::PrimField<f64>>,
    internal: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Container_Ports {
    #[doc =
        "Set the field `external`.\nPort exposed out of the container. If not given a free random port `>= 32768` will be used."]
    pub fn set_external(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.external = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\nIP address/mask that can access this port. Defaults to `0.0.0.0`."]
    pub fn set_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\nProtocol that can be used over this port. Defaults to `tcp`."]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Ports {
    type O = terrars::BlockAssignable<Container_Ports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Ports {
    #[doc = "Port within the container."]
    pub internal: terrars::PrimField<f64>,
}

impl BuildContainer_Ports {
    pub fn build(self) -> Container_Ports {
        Container_Ports {
            external: core::default::Default::default(),
            internal: self.internal,
            ip: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_PortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_PortsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_PortsRef {
        Container_PortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_PortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external` after provisioning.\nPort exposed out of the container. If not given a free random port `>= 32768` will be used."]
    pub fn external(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external", self.base))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\nPort within the container."]
    pub fn internal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nIP address/mask that can access this port. Defaults to `0.0.0.0`."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nProtocol that can be used over this port. Defaults to `tcp`."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Container_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Timeouts {
    type O = terrars::BlockAssignable<Container_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Timeouts {}

impl BuildContainer_Timeouts {
    pub fn build(self) -> Container_Timeouts {
        Container_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_TimeoutsRef {
        Container_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Ulimit {
    hard: terrars::PrimField<f64>,
    name: terrars::PrimField<std::string::String>,
    soft: terrars::PrimField<f64>,
}

impl Container_Ulimit { }

impl terrars::ToListMappable for Container_Ulimit {
    type O = terrars::BlockAssignable<Container_Ulimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Ulimit {
    #[doc = "The hard limit"]
    pub hard: terrars::PrimField<f64>,
    #[doc = "The name of the ulimit"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The soft limit"]
    pub soft: terrars::PrimField<f64>,
}

impl BuildContainer_Ulimit {
    pub fn build(self) -> Container_Ulimit {
        Container_Ulimit {
            hard: self.hard,
            name: self.name,
            soft: self.soft,
        }
    }
}

#[allow(dead_code)]
pub struct Container_UlimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_UlimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_UlimitRef {
        Container_UlimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_UlimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hard` after provisioning.\nThe hard limit"]
    pub fn hard(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hard", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the ulimit"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `soft` after provisioning.\nThe soft limit"]
    pub fn soft(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.soft", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Upload {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    executable: std::option::Option<terrars::PrimField<bool>>,
    file: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_hash: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Container_Upload {
    #[doc =
        "Set the field `content`.\nLiteral string value to use as the object content, which will be uploaded as UTF-8-encoded text. Conflicts with `content_base64` & `source`"]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_base64`.\nBase64-encoded data that will be decoded and uploaded as raw bytes for the object content. This allows safely uploading non-UTF8 binary data, but is recommended only for larger binary content such as the result of the `base64encode` interpolation function. See [here](https://github.com/terraform-providers/terraform-provider-docker/issues/48#issuecomment-374174588) for the reason. Conflicts with `content` & `source`"]
    pub fn set_content_base64(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_base64 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `executable`.\nIf `true`, the file will be uploaded with user executable permission. Defaults to `false`."]
    pub fn set_executable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.executable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `group`.\nThe group of the file in the container. Names are resolved using the Terraform host's group database; numeric IDs can be used directly."]
    pub fn set_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `owner`.\nThe owner of the file in the container. Names are resolved using the Terraform host's user database; numeric IDs can be used directly."]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permissions`.\nThe permission mode for the file in the container. Has precedence over `executable`."]
    pub fn set_permissions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source`.\nA filename that references a file which will be uploaded as the object content. This allows for large file uploads that do not get stored in state. Conflicts with `content` & `content_base64`"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_hash`.\nIf using `source`, this will force an update if the file content has updated but the filename has not. "]
    pub fn set_source_hash(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_hash = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Upload {
    type O = terrars::BlockAssignable<Container_Upload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Upload {
    #[doc = "Path to the file in the container where is upload goes to"]
    pub file: terrars::PrimField<std::string::String>,
}

impl BuildContainer_Upload {
    pub fn build(self) -> Container_Upload {
        Container_Upload {
            content: core::default::Default::default(),
            content_base64: core::default::Default::default(),
            executable: core::default::Default::default(),
            file: self.file,
            group: core::default::Default::default(),
            owner: core::default::Default::default(),
            permissions: core::default::Default::default(),
            source: core::default::Default::default(),
            source_hash: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_UploadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_UploadRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_UploadRef {
        Container_UploadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_UploadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nLiteral string value to use as the object content, which will be uploaded as UTF-8-encoded text. Conflicts with `content_base64` & `source`"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_base64` after provisioning.\nBase64-encoded data that will be decoded and uploaded as raw bytes for the object content. This allows safely uploading non-UTF8 binary data, but is recommended only for larger binary content such as the result of the `base64encode` interpolation function. See [here](https://github.com/terraform-providers/terraform-provider-docker/issues/48#issuecomment-374174588) for the reason. Conflicts with `content` & `source`"]
    pub fn content_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.base))
    }

    #[doc =
        "Get a reference to the value of field `executable` after provisioning.\nIf `true`, the file will be uploaded with user executable permission. Defaults to `false`."]
    pub fn executable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.executable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file` after provisioning.\nPath to the file in the container where is upload goes to"]
    pub fn file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group` after provisioning.\nThe group of the file in the container. Names are resolved using the Terraform host's group database; numeric IDs can be used directly."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `owner` after provisioning.\nThe owner of the file in the container. Names are resolved using the Terraform host's user database; numeric IDs can be used directly."]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nThe permission mode for the file in the container. Has precedence over `executable`."]
    pub fn permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nA filename that references a file which will be uploaded as the object content. This allows for large file uploads that do not get stored in state. Conflicts with `content` & `content_base64`"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_hash` after provisioning.\nIf using `source`, this will force an update if the file content has updated but the filename has not. "]
    pub fn source_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_hash", self.base))
    }
}

#[derive(Serialize)]
pub struct Container_Volumes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_container: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selinux_relabel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Container_Volumes {
    #[doc = "Set the field `container_path`.\nThe path in the container where the volume will be mounted."]
    pub fn set_container_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_container`.\nThe container where the volume is coming from."]
    pub fn set_from_container(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.from_container = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host_path`.\nThe path on the host where the volume is coming from. If `host_path` is set, it takes precedence over `volume_name`."]
    pub fn set_host_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\nIf `true`, this volume will be readonly. Defaults to `false`."]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `selinux_relabel`.\nSELinux relabel mode for bind mounts. Supported values are `z` and `Z`."]
    pub fn set_selinux_relabel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.selinux_relabel = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `volume_name`.\nThe name of the docker volume which should be mounted. Ignored when `host_path` is set."]
    pub fn set_volume_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Container_Volumes {
    type O = terrars::BlockAssignable<Container_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainer_Volumes {}

impl BuildContainer_Volumes {
    pub fn build(self) -> Container_Volumes {
        Container_Volumes {
            container_path: core::default::Default::default(),
            from_container: core::default::Default::default(),
            host_path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            selinux_relabel: core::default::Default::default(),
            volume_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Container_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Container_VolumesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Container_VolumesRef {
        Container_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Container_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_path` after provisioning.\nThe path in the container where the volume will be mounted."]
    pub fn container_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `from_container` after provisioning.\nThe container where the volume is coming from."]
    pub fn from_container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_container", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_path` after provisioning.\nThe path on the host where the volume is coming from. If `host_path` is set, it takes precedence over `volume_name`."]
    pub fn host_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIf `true`, this volume will be readonly. Defaults to `false`."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `selinux_relabel` after provisioning.\nSELinux relabel mode for bind mounts. Supported values are `z` and `Z`."]
    pub fn selinux_relabel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selinux_relabel", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_name` after provisioning.\nThe name of the docker volume which should be mounted. Ignored when `host_path` is set."]
    pub fn volume_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_name", self.base))
    }
}
