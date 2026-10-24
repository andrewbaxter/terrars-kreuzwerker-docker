use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct ContainerData {
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
    cgroupns_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_read_refresh_timeout_milliseconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_shares: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destroy_grace_seconds: std::option::Option<terrars::PrimField<f64>>,
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
    memory_swap: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    must_run: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid_mode: std::option::Option<terrars::PrimField<std::string::String>>,
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
    capabilities: std::option::Option<std::vec::Vec<ContainerCapabilitiesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    devices: std::option::Option<std::vec::Vec<ContainerDevicesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthcheck: std::option::Option<std::vec::Vec<ContainerHealthcheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<std::vec::Vec<ContainerHostEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<ContainerLabelsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mounts: std::option::Option<std::vec::Vec<ContainerMountsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks_advanced: std::option::Option<std::vec::Vec<ContainerNetworksAdvancedEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<ContainerPortsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ulimit: std::option::Option<std::vec::Vec<ContainerUlimitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload: std::option::Option<std::vec::Vec<ContainerUploadEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<ContainerVolumesEl>>,
    dynamic: ContainerDynamic,
}

struct Container_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerData>,
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

    #[doc =
        "Set the field `cgroupns_mode`.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn set_cgroupns_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cgroupns_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `command`.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.con\"]`."]
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
        "Set the field `destroy_grace_seconds`.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn set_destroy_grace_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().destroy_grace_seconds = std::option::Option::Some(v.into());
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
        "Set the field `entrypoint`.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogra\"]`."]
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
        "Set the field `gpus`.\nGPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior."]
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
        "Set the field `memory_swap`.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn set_memory_swap(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory_swap = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `must_run`.\nIf `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform assumes it is successful. Defaults to `true`."]
    pub fn set_must_run(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().must_run = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_mode`.\nNetwork mode of the container."]
    pub fn set_network_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pid_mode`.\nhe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn set_pid_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pid_mode = std::option::Option::Some(v.into());
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
        "Set the field `user`.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name."]
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
        "Set the field `wait`.\nIf `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`."]
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
    pub fn set_capabilities(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerCapabilitiesEl>>,
    ) -> Self {
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

    #[doc = "Set the field `devices`.\n"]
    pub fn set_devices(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerDevicesEl>>) -> Self {
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
    pub fn set_healthcheck(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerHealthcheckEl>>) -> Self {
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
    pub fn set_host(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerHostEl>>) -> Self {
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
    pub fn set_labels(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerLabelsEl>>) -> Self {
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
    pub fn set_mounts(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerMountsEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerNetworksAdvancedEl>>,
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
    pub fn set_ports(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerPortsEl>>) -> Self {
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

    #[doc = "Set the field `ulimit`.\n"]
    pub fn set_ulimit(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerUlimitEl>>) -> Self {
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
    pub fn set_upload(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerUploadEl>>) -> Self {
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
    pub fn set_volumes(self, v: impl std::convert::Into<terrars::BlockAssignable<ContainerVolumesEl>>) -> Self {
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
        "Get a reference to the value of field `cgroupns_mode` after provisioning.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn cgroupns_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroupns_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.con\"]`."]
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
        "Get a reference to the value of field `destroy_grace_seconds` after provisioning.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn destroy_grace_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destroy_grace_seconds", self.extract_ref()))
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
        "Get a reference to the value of field `entrypoint` after provisioning.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogra\"]`."]
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
        "Get a reference to the value of field `gpus` after provisioning.\nGPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior."]
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
        "Get a reference to the value of field `image` after provisioning.\nThe ID of the image to back this container. The easiest way to get this value is to use the `docker_image` resource as is shown in the example."]
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
        "Get a reference to the value of field `memory_swap` after provisioning.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn memory_swap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `must_run` after provisioning.\nIf `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform assumes it is successful. Defaults to `true`."]
    pub fn must_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.must_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the container."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_data` after provisioning.\nThe data of the networks the container is connected to."]
    pub fn network_data(&self) -> terrars::ListRef<ContainerNetworkDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\nNetwork mode of the container."]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pid_mode` after provisioning.\nhe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn pid_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
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
        "Get a reference to the value of field `user` after provisioning.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `userns_mode` after provisioning.\nSets the usernamespace mode for the container when usernamespace remapping option is enabled."]
    pub fn userns_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.userns_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nIf `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`."]
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
    pub fn healthcheck(&self) -> terrars::ListRef<ContainerHealthcheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<ContainerPortsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
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
        "The ID of the image to back this container. The easiest way to get this value is to use the `docker_image` resource as is shown in the example."]
    pub image: terrars::PrimField<std::string::String>,
    #[doc = "The name of the container."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildContainer {
    pub fn build(self, stack: &mut terrars::Stack) -> Container {
        let out = Container(std::rc::Rc::new(Container_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                attach: core::default::Default::default(),
                cgroupns_mode: core::default::Default::default(),
                command: core::default::Default::default(),
                container_read_refresh_timeout_milliseconds: core::default::Default::default(),
                cpu_set: core::default::Default::default(),
                cpu_shares: core::default::Default::default(),
                destroy_grace_seconds: core::default::Default::default(),
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
                memory_swap: core::default::Default::default(),
                must_run: core::default::Default::default(),
                name: self.name,
                network_mode: core::default::Default::default(),
                pid_mode: core::default::Default::default(),
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
                devices: core::default::Default::default(),
                healthcheck: core::default::Default::default(),
                host: core::default::Default::default(),
                labels: core::default::Default::default(),
                mounts: core::default::Default::default(),
                networks_advanced: core::default::Default::default(),
                ports: core::default::Default::default(),
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
        "Get a reference to the value of field `cgroupns_mode` after provisioning.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn cgroupns_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cgroupns_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.con\"]`."]
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
        "Get a reference to the value of field `destroy_grace_seconds` after provisioning.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn destroy_grace_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destroy_grace_seconds", self.extract_ref()))
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
        "Get a reference to the value of field `entrypoint` after provisioning.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogra\"]`."]
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
        "Get a reference to the value of field `gpus` after provisioning.\nGPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior."]
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
        "Get a reference to the value of field `image` after provisioning.\nThe ID of the image to back this container. The easiest way to get this value is to use the `docker_image` resource as is shown in the example."]
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
        "Get a reference to the value of field `memory_swap` after provisioning.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn memory_swap(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `must_run` after provisioning.\nIf `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform assumes it is successful. Defaults to `true`."]
    pub fn must_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.must_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the container."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_data` after provisioning.\nThe data of the networks the container is connected to."]
    pub fn network_data(&self) -> terrars::ListRef<ContainerNetworkDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\nNetwork mode of the container."]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pid_mode` after provisioning.\nhe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn pid_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
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
        "Get a reference to the value of field `user` after provisioning.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `userns_mode` after provisioning.\nSets the usernamespace mode for the container when usernamespace remapping option is enabled."]
    pub fn userns_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.userns_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nIf `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`."]
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
    pub fn healthcheck(&self) -> terrars::ListRef<ContainerHealthcheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<ContainerPortsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ContainerNetworkDataEl {
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

impl ContainerNetworkDataEl {
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

impl terrars::ToListMappable for ContainerNetworkDataEl {
    type O = terrars::BlockAssignable<ContainerNetworkDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNetworkDataEl {}

impl BuildContainerNetworkDataEl {
    pub fn build(self) -> ContainerNetworkDataEl {
        ContainerNetworkDataEl {
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

pub struct ContainerNetworkDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNetworkDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNetworkDataElRef {
        ContainerNetworkDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNetworkDataElRef {
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
pub struct ContainerCapabilitiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ContainerCapabilitiesEl {
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

impl terrars::ToListMappable for ContainerCapabilitiesEl {
    type O = terrars::BlockAssignable<ContainerCapabilitiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerCapabilitiesEl {}

impl BuildContainerCapabilitiesEl {
    pub fn build(self) -> ContainerCapabilitiesEl {
        ContainerCapabilitiesEl {
            add: core::default::Default::default(),
            drop: core::default::Default::default(),
        }
    }
}

pub struct ContainerCapabilitiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerCapabilitiesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerCapabilitiesElRef {
        ContainerCapabilitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCapabilitiesElRef {
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
pub struct ContainerDevicesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    host_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerDevicesEl {
    #[doc = "Set the field `container_path`.\nThe path in the container where the device will be bound."]
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

impl terrars::ToListMappable for ContainerDevicesEl {
    type O = terrars::BlockAssignable<ContainerDevicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerDevicesEl {
    #[doc = "The path on the host where the device is located."]
    pub host_path: terrars::PrimField<std::string::String>,
}

impl BuildContainerDevicesEl {
    pub fn build(self) -> ContainerDevicesEl {
        ContainerDevicesEl {
            container_path: core::default::Default::default(),
            host_path: self.host_path,
            permissions: core::default::Default::default(),
        }
    }
}

pub struct ContainerDevicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerDevicesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerDevicesElRef {
        ContainerDevicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerDevicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_path` after provisioning.\nThe path in the container where the device will be bound."]
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
pub struct ContainerHealthcheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_period: std::option::Option<terrars::PrimField<std::string::String>>,
    test: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerHealthcheckEl {
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

    #[doc =
        "Set the field `start_period`.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn set_start_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerHealthcheckEl {
    type O = terrars::BlockAssignable<ContainerHealthcheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerHealthcheckEl {
    #[doc =
        "Command to run to check health. For example, to run `curl -f localhost/health` set the command to be `[\"CMD\", \"curl\", \"-f\", \"localhost/health\"]`."]
    pub test: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildContainerHealthcheckEl {
    pub fn build(self) -> ContainerHealthcheckEl {
        ContainerHealthcheckEl {
            interval: core::default::Default::default(),
            retries: core::default::Default::default(),
            start_period: core::default::Default::default(),
            test: self.test,
            timeout: core::default::Default::default(),
        }
    }
}

pub struct ContainerHealthcheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerHealthcheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerHealthcheckElRef {
        ContainerHealthcheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerHealthcheckElRef {
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
        "Get a reference to the value of field `start_period` after provisioning.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn start_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `test` after provisioning.\nCommand to run to check health. For example, to run `curl -f localhost/health` set the command to be `[\"CMD\", \"curl\", \"-f\", \"localhost/health\"]`."]
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
pub struct ContainerHostEl {
    host: terrars::PrimField<std::string::String>,
    ip: terrars::PrimField<std::string::String>,
}

impl ContainerHostEl { }

impl terrars::ToListMappable for ContainerHostEl {
    type O = terrars::BlockAssignable<ContainerHostEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerHostEl {
    #[doc = "Hostname to add"]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "IP address this hostname should resolve to."]
    pub ip: terrars::PrimField<std::string::String>,
}

impl BuildContainerHostEl {
    pub fn build(self) -> ContainerHostEl {
        ContainerHostEl {
            host: self.host,
            ip: self.ip,
        }
    }
}

pub struct ContainerHostElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerHostElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerHostElRef {
        ContainerHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerHostElRef {
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
pub struct ContainerLabelsEl {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ContainerLabelsEl { }

impl terrars::ToListMappable for ContainerLabelsEl {
    type O = terrars::BlockAssignable<ContainerLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerLabelsEl {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildContainerLabelsEl {
    pub fn build(self) -> ContainerLabelsEl {
        ContainerLabelsEl {
            label: self.label,
            value: self.value,
        }
    }
}

pub struct ContainerLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerLabelsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerLabelsElRef {
        ContainerLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerLabelsElRef {
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
pub struct ContainerMountsElBindOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerMountsElBindOptionsEl {
    #[doc = "Set the field `propagation`.\nA propagation mode with the value."]
    pub fn set_propagation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.propagation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerMountsElBindOptionsEl {
    type O = terrars::BlockAssignable<ContainerMountsElBindOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerMountsElBindOptionsEl {}

impl BuildContainerMountsElBindOptionsEl {
    pub fn build(self) -> ContainerMountsElBindOptionsEl {
        ContainerMountsElBindOptionsEl { propagation: core::default::Default::default() }
    }
}

pub struct ContainerMountsElBindOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerMountsElBindOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerMountsElBindOptionsElRef {
        ContainerMountsElBindOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElBindOptionsElRef {
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
pub struct ContainerMountsElTmpfsOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_bytes: std::option::Option<terrars::PrimField<f64>>,
}

impl ContainerMountsElTmpfsOptionsEl {
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

impl terrars::ToListMappable for ContainerMountsElTmpfsOptionsEl {
    type O = terrars::BlockAssignable<ContainerMountsElTmpfsOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerMountsElTmpfsOptionsEl {}

impl BuildContainerMountsElTmpfsOptionsEl {
    pub fn build(self) -> ContainerMountsElTmpfsOptionsEl {
        ContainerMountsElTmpfsOptionsEl {
            mode: core::default::Default::default(),
            size_bytes: core::default::Default::default(),
        }
    }
}

pub struct ContainerMountsElTmpfsOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerMountsElTmpfsOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerMountsElTmpfsOptionsElRef {
        ContainerMountsElTmpfsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElTmpfsOptionsElRef {
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

#[derive(Serialize)]
pub struct ContainerMountsElVolumeOptionsElLabelsEl {
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ContainerMountsElVolumeOptionsElLabelsEl { }

impl terrars::ToListMappable for ContainerMountsElVolumeOptionsElLabelsEl {
    type O = terrars::BlockAssignable<ContainerMountsElVolumeOptionsElLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerMountsElVolumeOptionsElLabelsEl {
    #[doc = "Name of the label"]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = "Value of the label"]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildContainerMountsElVolumeOptionsElLabelsEl {
    pub fn build(self) -> ContainerMountsElVolumeOptionsElLabelsEl {
        ContainerMountsElVolumeOptionsElLabelsEl {
            label: self.label,
            value: self.value,
        }
    }
}

pub struct ContainerMountsElVolumeOptionsElLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerMountsElVolumeOptionsElLabelsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerMountsElVolumeOptionsElLabelsElRef {
        ContainerMountsElVolumeOptionsElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElVolumeOptionsElLabelsElRef {
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

#[derive(Serialize, Default)]
struct ContainerMountsElVolumeOptionsElDynamic {
    labels: std::option::Option<terrars::DynamicBlock<ContainerMountsElVolumeOptionsElLabelsEl>>,
}

#[derive(Serialize)]
pub struct ContainerMountsElVolumeOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_copy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<ContainerMountsElVolumeOptionsElLabelsEl>>,
    dynamic: ContainerMountsElVolumeOptionsElDynamic,
}

impl ContainerMountsElVolumeOptionsEl {
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

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerMountsElVolumeOptionsElLabelsEl>>,
    ) -> Self {
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

impl terrars::ToListMappable for ContainerMountsElVolumeOptionsEl {
    type O = terrars::BlockAssignable<ContainerMountsElVolumeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerMountsElVolumeOptionsEl {}

impl BuildContainerMountsElVolumeOptionsEl {
    pub fn build(self) -> ContainerMountsElVolumeOptionsEl {
        ContainerMountsElVolumeOptionsEl {
            driver_name: core::default::Default::default(),
            driver_options: core::default::Default::default(),
            no_copy: core::default::Default::default(),
            labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerMountsElVolumeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerMountsElVolumeOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerMountsElVolumeOptionsElRef {
        ContainerMountsElVolumeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElVolumeOptionsElRef {
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
}

#[derive(Serialize, Default)]
struct ContainerMountsElDynamic {
    bind_options: std::option::Option<terrars::DynamicBlock<ContainerMountsElBindOptionsEl>>,
    tmpfs_options: std::option::Option<terrars::DynamicBlock<ContainerMountsElTmpfsOptionsEl>>,
    volume_options: std::option::Option<terrars::DynamicBlock<ContainerMountsElVolumeOptionsEl>>,
}

#[derive(Serialize)]
pub struct ContainerMountsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bind_options: std::option::Option<std::vec::Vec<ContainerMountsElBindOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tmpfs_options: std::option::Option<std::vec::Vec<ContainerMountsElTmpfsOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_options: std::option::Option<std::vec::Vec<ContainerMountsElVolumeOptionsEl>>,
    dynamic: ContainerMountsElDynamic,
}

impl ContainerMountsEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerMountsElBindOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerMountsElTmpfsOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerMountsElVolumeOptionsEl>>,
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

impl terrars::ToListMappable for ContainerMountsEl {
    type O = terrars::BlockAssignable<ContainerMountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerMountsEl {
    #[doc = "Container path"]
    pub target: terrars::PrimField<std::string::String>,
    #[doc = "The mount type"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildContainerMountsEl {
    pub fn build(self) -> ContainerMountsEl {
        ContainerMountsEl {
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

pub struct ContainerMountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerMountsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerMountsElRef {
        ContainerMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElRef {
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
    pub fn bind_options(&self) -> terrars::ListRef<ContainerMountsElBindOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bind_options", self.base))
    }

    #[doc = "Get a reference to the value of field `tmpfs_options` after provisioning.\n"]
    pub fn tmpfs_options(&self) -> terrars::ListRef<ContainerMountsElTmpfsOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tmpfs_options", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_options` after provisioning.\n"]
    pub fn volume_options(&self) -> terrars::ListRef<ContainerMountsElVolumeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNetworksAdvancedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aliases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl ContainerNetworksAdvancedEl {
    #[doc = "Set the field `aliases`.\nThe network aliases of the container in the specific network."]
    pub fn set_aliases(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aliases = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for ContainerNetworksAdvancedEl {
    type O = terrars::BlockAssignable<ContainerNetworksAdvancedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNetworksAdvancedEl {
    #[doc =
        "The name or id of the network to use. You can use `name` or `id` attribute from a `docker_network` resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildContainerNetworksAdvancedEl {
    pub fn build(self) -> ContainerNetworksAdvancedEl {
        ContainerNetworksAdvancedEl {
            aliases: core::default::Default::default(),
            ipv4_address: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct ContainerNetworksAdvancedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerNetworksAdvancedElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerNetworksAdvancedElRef {
        ContainerNetworksAdvancedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNetworksAdvancedElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aliases` after provisioning.\nThe network aliases of the container in the specific network."]
    pub fn aliases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aliases", self.base))
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
        "Get a reference to the value of field `name` after provisioning.\nThe name or id of the network to use. You can use `name` or `id` attribute from a `docker_network` resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerPortsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external: std::option::Option<terrars::PrimField<f64>>,
    internal: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerPortsEl {
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

impl terrars::ToListMappable for ContainerPortsEl {
    type O = terrars::BlockAssignable<ContainerPortsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerPortsEl {
    #[doc = "Port within the container."]
    pub internal: terrars::PrimField<f64>,
}

impl BuildContainerPortsEl {
    pub fn build(self) -> ContainerPortsEl {
        ContainerPortsEl {
            external: core::default::Default::default(),
            internal: self.internal,
            ip: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

pub struct ContainerPortsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerPortsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerPortsElRef {
        ContainerPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerPortsElRef {
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
pub struct ContainerUlimitEl {
    hard: terrars::PrimField<f64>,
    name: terrars::PrimField<std::string::String>,
    soft: terrars::PrimField<f64>,
}

impl ContainerUlimitEl { }

impl terrars::ToListMappable for ContainerUlimitEl {
    type O = terrars::BlockAssignable<ContainerUlimitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerUlimitEl {
    #[doc = "The hard limit"]
    pub hard: terrars::PrimField<f64>,
    #[doc = "The name of the ulimit"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The soft limit"]
    pub soft: terrars::PrimField<f64>,
}

impl BuildContainerUlimitEl {
    pub fn build(self) -> ContainerUlimitEl {
        ContainerUlimitEl {
            hard: self.hard,
            name: self.name,
            soft: self.soft,
        }
    }
}

pub struct ContainerUlimitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerUlimitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerUlimitElRef {
        ContainerUlimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerUlimitElRef {
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
pub struct ContainerUploadEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    executable: std::option::Option<terrars::PrimField<bool>>,
    file: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_hash: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerUploadEl {
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

impl terrars::ToListMappable for ContainerUploadEl {
    type O = terrars::BlockAssignable<ContainerUploadEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerUploadEl {
    #[doc = "Path to the file in the container where is upload goes to"]
    pub file: terrars::PrimField<std::string::String>,
}

impl BuildContainerUploadEl {
    pub fn build(self) -> ContainerUploadEl {
        ContainerUploadEl {
            content: core::default::Default::default(),
            content_base64: core::default::Default::default(),
            executable: core::default::Default::default(),
            file: self.file,
            source: core::default::Default::default(),
            source_hash: core::default::Default::default(),
        }
    }
}

pub struct ContainerUploadElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerUploadElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerUploadElRef {
        ContainerUploadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerUploadElRef {
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
pub struct ContainerVolumesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_container: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerVolumesEl {
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

    #[doc = "Set the field `host_path`.\nThe path on the host where the volume is coming from."]
    pub fn set_host_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\nIf `true`, this volume will be readonly. Defaults to `false`."]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_name`.\nThe name of the docker volume which should be mounted."]
    pub fn set_volume_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerVolumesEl {
    type O = terrars::BlockAssignable<ContainerVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerVolumesEl {}

impl BuildContainerVolumesEl {
    pub fn build(self) -> ContainerVolumesEl {
        ContainerVolumesEl {
            container_path: core::default::Default::default(),
            from_container: core::default::Default::default(),
            host_path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            volume_name: core::default::Default::default(),
        }
    }
}

pub struct ContainerVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerVolumesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerVolumesElRef {
        ContainerVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerVolumesElRef {
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
        "Get a reference to the value of field `host_path` after provisioning.\nThe path on the host where the volume is coming from."]
    pub fn host_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIf `true`, this volume will be readonly. Defaults to `false`."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_name` after provisioning.\nThe name of the docker volume which should be mounted."]
    pub fn volume_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerDynamic {
    capabilities: std::option::Option<terrars::DynamicBlock<ContainerCapabilitiesEl>>,
    devices: std::option::Option<terrars::DynamicBlock<ContainerDevicesEl>>,
    healthcheck: std::option::Option<terrars::DynamicBlock<ContainerHealthcheckEl>>,
    host: std::option::Option<terrars::DynamicBlock<ContainerHostEl>>,
    labels: std::option::Option<terrars::DynamicBlock<ContainerLabelsEl>>,
    mounts: std::option::Option<terrars::DynamicBlock<ContainerMountsEl>>,
    networks_advanced: std::option::Option<terrars::DynamicBlock<ContainerNetworksAdvancedEl>>,
    ports: std::option::Option<terrars::DynamicBlock<ContainerPortsEl>>,
    ulimit: std::option::Option<terrars::DynamicBlock<ContainerUlimitEl>>,
    upload: std::option::Option<terrars::DynamicBlock<ContainerUploadEl>>,
    volumes: std::option::Option<terrars::DynamicBlock<ContainerVolumesEl>>,
}
