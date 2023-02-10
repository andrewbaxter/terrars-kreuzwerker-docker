use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDocker;

#[derive(Serialize)]
struct ContainerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attach: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroupns_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_read_refresh_timeout_milliseconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_set: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_shares: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destroy_grace_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_opts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_search: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domainname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpus: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_add: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    init: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipc_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_driver: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_opts: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logs: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retry_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_swap: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    must_run: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publish_all_ports: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_volumes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rm: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_opts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shm_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdin_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_signal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_opts: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sysctls: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tmpfs: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tty: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    userns_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<Vec<ContainerCapabilitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    devices: Option<Vec<ContainerDevicesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthcheck: Option<Vec<ContainerHealthcheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<Vec<ContainerHostEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<ContainerLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mounts: Option<Vec<ContainerMountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks_advanced: Option<Vec<ContainerNetworksAdvancedEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<Vec<ContainerPortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ulimit: Option<Vec<ContainerUlimitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload: Option<Vec<ContainerUploadEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<ContainerVolumesEl>>,
    dynamic: ContainerDynamic,
}

struct Container_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerData>,
}

#[derive(Clone)]
pub struct Container(Rc<Container_>);

impl Container {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `attach`.\nIf `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`."]
    pub fn set_attach(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().attach = Some(v.into());
        self
    }

    #[doc= "Set the field `cgroupns_mode`.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn set_cgroupns_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cgroupns_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `command`.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.con\"]`."]
    pub fn set_command(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().command = Some(v.into());
        self
    }

    #[doc= "Set the field `container_read_refresh_timeout_milliseconds`.\nThe total number of milliseconds to wait for the container to reach status 'running'"]
    pub fn set_container_read_refresh_timeout_milliseconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().container_read_refresh_timeout_milliseconds = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_set`.\nA comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`."]
    pub fn set_cpu_set(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cpu_set = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_shares`.\nCPU shares (relative weight) for the container."]
    pub fn set_cpu_shares(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpu_shares = Some(v.into());
        self
    }

    #[doc= "Set the field `destroy_grace_seconds`.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn set_destroy_grace_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().destroy_grace_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `dns`.\nDNS servers to use."]
    pub fn set_dns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().dns = Some(v.into());
        self
    }

    #[doc= "Set the field `dns_opts`.\nDNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options."]
    pub fn set_dns_opts(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().dns_opts = Some(v.into());
        self
    }

    #[doc= "Set the field `dns_search`.\nDNS search domains that are used when bare unqualified hostnames are used inside of the container."]
    pub fn set_dns_search(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().dns_search = Some(v.into());
        self
    }

    #[doc= "Set the field `domainname`.\nDomain name of the container."]
    pub fn set_domainname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domainname = Some(v.into());
        self
    }

    #[doc= "Set the field `entrypoint`.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogra\"]`."]
    pub fn set_entrypoint(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().entrypoint = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\nEnvironment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn set_env(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().env = Some(v.into());
        self
    }

    #[doc= "Set the field `gpus`.\nGPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior."]
    pub fn set_gpus(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gpus = Some(v.into());
        self
    }

    #[doc= "Set the field `group_add`.\nAdditional groups for the container user"]
    pub fn set_group_add(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().group_add = Some(v.into());
        self
    }

    #[doc= "Set the field `hostname`.\nHostname of the container."]
    pub fn set_hostname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hostname = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `init`.\nConfigured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults."]
    pub fn set_init(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().init = Some(v.into());
        self
    }

    #[doc= "Set the field `ipc_mode`.\nIPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`."]
    pub fn set_ipc_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipc_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `log_driver`.\nThe logging driver to use for the container."]
    pub fn set_log_driver(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().log_driver = Some(v.into());
        self
    }

    #[doc= "Set the field `log_opts`.\nKey/value pairs to use as options for the logging driver."]
    pub fn set_log_opts(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().log_opts = Some(v.into());
        self
    }

    #[doc= "Set the field `logs`.\nSave the container logs (`attach` must be enabled). Defaults to `false`."]
    pub fn set_logs(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().logs = Some(v.into());
        self
    }

    #[doc= "Set the field `max_retry_count`.\nThe maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'."]
    pub fn set_max_retry_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_retry_count = Some(v.into());
        self
    }

    #[doc= "Set the field `memory`.\nThe memory limit for the container in MBs."]
    pub fn set_memory(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory = Some(v.into());
        self
    }

    #[doc= "Set the field `memory_swap`.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn set_memory_swap(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory_swap = Some(v.into());
        self
    }

    #[doc= "Set the field `must_run`.\nIf `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform assumes it is successful. Defaults to `true`."]
    pub fn set_must_run(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().must_run = Some(v.into());
        self
    }

    #[doc= "Set the field `network_mode`.\nNetwork mode of the container."]
    pub fn set_network_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `pid_mode`.\nhe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn set_pid_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pid_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `privileged`.\nIf `true`, the container runs in privileged mode."]
    pub fn set_privileged(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().privileged = Some(v.into());
        self
    }

    #[doc= "Set the field `publish_all_ports`.\nPublish all ports of the container."]
    pub fn set_publish_all_ports(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publish_all_ports = Some(v.into());
        self
    }

    #[doc= "Set the field `read_only`.\nIf `true`, the container will be started as readonly. Defaults to `false`."]
    pub fn set_read_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().read_only = Some(v.into());
        self
    }

    #[doc= "Set the field `remove_volumes`.\nIf `true`, it will remove anonymous volumes associated with the container. Defaults to `true`."]
    pub fn set_remove_volumes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_volumes = Some(v.into());
        self
    }

    #[doc= "Set the field `restart`.\nThe restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`."]
    pub fn set_restart(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().restart = Some(v.into());
        self
    }

    #[doc= "Set the field `rm`.\nIf `true`, then the container will be automatically removed when it exits. Defaults to `false`."]
    pub fn set_rm(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().rm = Some(v.into());
        self
    }

    #[doc= "Set the field `runtime`.\nRuntime to use for the container."]
    pub fn set_runtime(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().runtime = Some(v.into());
        self
    }

    #[doc= "Set the field `security_opts`.\nList of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration."]
    pub fn set_security_opts(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_opts = Some(v.into());
        self
    }

    #[doc= "Set the field `shm_size`.\nSize of `/dev/shm` in MBs."]
    pub fn set_shm_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().shm_size = Some(v.into());
        self
    }

    #[doc= "Set the field `start`.\nIf `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`."]
    pub fn set_start(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start = Some(v.into());
        self
    }

    #[doc= "Set the field `stdin_open`.\nIf `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`."]
    pub fn set_stdin_open(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().stdin_open = Some(v.into());
        self
    }

    #[doc= "Set the field `stop_signal`.\nSignal to stop a container (default `SIGTERM`)."]
    pub fn set_stop_signal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stop_signal = Some(v.into());
        self
    }

    #[doc= "Set the field `stop_timeout`.\nTimeout (in seconds) to stop a container."]
    pub fn set_stop_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().stop_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `storage_opts`.\nKey/value pairs for the storage driver options, e.g. `size`: `120G`"]
    pub fn set_storage_opts(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().storage_opts = Some(v.into());
        self
    }

    #[doc= "Set the field `sysctls`.\nA map of kernel parameters (sysctls) to set in the container."]
    pub fn set_sysctls(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().sysctls = Some(v.into());
        self
    }

    #[doc= "Set the field `tmpfs`.\nA map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options."]
    pub fn set_tmpfs(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tmpfs = Some(v.into());
        self
    }

    #[doc= "Set the field `tty`.\nIf `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`."]
    pub fn set_tty(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tty = Some(v.into());
        self
    }

    #[doc= "Set the field `user`.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name."]
    pub fn set_user(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user = Some(v.into());
        self
    }

    #[doc= "Set the field `userns_mode`.\nSets the usernamespace mode for the container when usernamespace remapping option is enabled."]
    pub fn set_userns_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().userns_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `wait`.\nIf `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`."]
    pub fn set_wait(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait = Some(v.into());
        self
    }

    #[doc= "Set the field `wait_timeout`.\nThe timeout in seconds to wait the container to be healthy after creation. Defaults to `60`."]
    pub fn set_wait_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wait_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `working_dir`.\nThe working directory for commands to run in."]
    pub fn set_working_dir(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().working_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `capabilities`.\n"]
    pub fn set_capabilities(self, v: impl Into<BlockAssignable<ContainerCapabilitiesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capabilities = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capabilities = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `devices`.\n"]
    pub fn set_devices(self, v: impl Into<BlockAssignable<ContainerDevicesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().devices = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.devices = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `healthcheck`.\n"]
    pub fn set_healthcheck(self, v: impl Into<BlockAssignable<ContainerHealthcheckEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().healthcheck = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.healthcheck = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `host`.\n"]
    pub fn set_host(self, v: impl Into<BlockAssignable<ContainerHostEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().host = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.host = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl Into<BlockAssignable<ContainerLabelsEl>>) -> Self {
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

    #[doc= "Set the field `mounts`.\n"]
    pub fn set_mounts(self, v: impl Into<BlockAssignable<ContainerMountsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mounts = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mounts = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `networks_advanced`.\n"]
    pub fn set_networks_advanced(self, v: impl Into<BlockAssignable<ContainerNetworksAdvancedEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks_advanced = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks_advanced = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ports`.\n"]
    pub fn set_ports(self, v: impl Into<BlockAssignable<ContainerPortsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ports = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ports = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ulimit`.\n"]
    pub fn set_ulimit(self, v: impl Into<BlockAssignable<ContainerUlimitEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ulimit = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ulimit = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `upload`.\n"]
    pub fn set_upload(self, v: impl Into<BlockAssignable<ContainerUploadEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upload = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upload = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `volumes`.\n"]
    pub fn set_volumes(self, v: impl Into<BlockAssignable<ContainerVolumesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().volumes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.volumes = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `attach` after provisioning.\nIf `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`."]
    pub fn attach(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.attach", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bridge` after provisioning.\nThe network bridge of the container as read from its NetworkSettings."]
    pub fn bridge(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bridge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cgroupns_mode` after provisioning.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn cgroupns_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroupns_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `command` after provisioning.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.con\"]`."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_logs` after provisioning.\nThe logs of the container if its execution is done (`attach` must be disabled)."]
    pub fn container_logs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_logs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_read_refresh_timeout_milliseconds` after provisioning.\nThe total number of milliseconds to wait for the container to reach status 'running'"]
    pub fn container_read_refresh_timeout_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_read_refresh_timeout_milliseconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `cpu_set` after provisioning.\nA comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`."]
    pub fn cpu_set(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_set", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cpu_shares` after provisioning.\nCPU shares (relative weight) for the container."]
    pub fn cpu_shares(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_shares", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destroy_grace_seconds` after provisioning.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn destroy_grace_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.destroy_grace_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns` after provisioning.\nDNS servers to use."]
    pub fn dns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_opts` after provisioning.\nDNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options."]
    pub fn dns_opts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_search` after provisioning.\nDNS search domains that are used when bare unqualified hostnames are used inside of the container."]
    pub fn dns_search(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domainname` after provisioning.\nDomain name of the container."]
    pub fn domainname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domainname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `entrypoint` after provisioning.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogra\"]`."]
    pub fn entrypoint(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nEnvironment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exit_code` after provisioning.\nThe exit code of the container if its execution is done (`must_run` must be disabled)."]
    pub fn exit_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.exit_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gpus` after provisioning.\nGPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior."]
    pub fn gpus(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_add` after provisioning.\nAdditional groups for the container user"]
    pub fn group_add(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.group_add", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nHostname of the container."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nThe ID of the image to back this container. The easiest way to get this value is to use the `docker_image` resource as is shown in the example."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `init` after provisioning.\nConfigured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults."]
    pub fn init(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.init", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipc_mode` after provisioning.\nIPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`."]
    pub fn ipc_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_driver` after provisioning.\nThe logging driver to use for the container."]
    pub fn log_driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_opts` after provisioning.\nKey/value pairs to use as options for the logging driver."]
    pub fn log_opts(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.log_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logs` after provisioning.\nSave the container logs (`attach` must be enabled). Defaults to `false`."]
    pub fn logs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_retry_count` after provisioning.\nThe maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'."]
    pub fn max_retry_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retry_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\nThe memory limit for the container in MBs."]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory_swap` after provisioning.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn memory_swap(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `must_run` after provisioning.\nIf `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform assumes it is successful. Defaults to `true`."]
    pub fn must_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.must_run", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the container."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_data` after provisioning.\nThe data of the networks the container is connected to."]
    pub fn network_data(&self) -> ListRef<ContainerNetworkDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_mode` after provisioning.\nNetwork mode of the container."]
    pub fn network_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pid_mode` after provisioning.\nhe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn pid_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `privileged` after provisioning.\nIf `true`, the container runs in privileged mode."]
    pub fn privileged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `publish_all_ports` after provisioning.\nPublish all ports of the container."]
    pub fn publish_all_ports(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publish_all_ports", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `read_only` after provisioning.\nIf `true`, the container will be started as readonly. Defaults to `false`."]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_volumes` after provisioning.\nIf `true`, it will remove anonymous volumes associated with the container. Defaults to `true`."]
    pub fn remove_volumes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_volumes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restart` after provisioning.\nThe restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`."]
    pub fn restart(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restart", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rm` after provisioning.\nIf `true`, then the container will be automatically removed when it exits. Defaults to `false`."]
    pub fn rm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runtime` after provisioning.\nRuntime to use for the container."]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_opts` after provisioning.\nList of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration."]
    pub fn security_opts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shm_size` after provisioning.\nSize of `/dev/shm` in MBs."]
    pub fn shm_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shm_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start` after provisioning.\nIf `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`."]
    pub fn start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stdin_open` after provisioning.\nIf `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`."]
    pub fn stdin_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.stdin_open", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to stop a container (default `SIGTERM`)."]
    pub fn stop_signal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stop_timeout` after provisioning.\nTimeout (in seconds) to stop a container."]
    pub fn stop_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.stop_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_opts` after provisioning.\nKey/value pairs for the storage driver options, e.g. `size`: `120G`"]
    pub fn storage_opts(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.storage_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sysctls` after provisioning.\nA map of kernel parameters (sysctls) to set in the container."]
    pub fn sysctls(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tmpfs` after provisioning.\nA map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options."]
    pub fn tmpfs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tmpfs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tty` after provisioning.\nIf `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`."]
    pub fn tty(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tty", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name."]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `userns_mode` after provisioning.\nSets the usernamespace mode for the container when usernamespace remapping option is enabled."]
    pub fn userns_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.userns_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait` after provisioning.\nIf `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`."]
    pub fn wait(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_timeout` after provisioning.\nThe timeout in seconds to wait the container to be healthy after creation. Defaults to `60`."]
    pub fn wait_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `working_dir` after provisioning.\nThe working directory for commands to run in."]
    pub fn working_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> ListRef<ContainerHealthcheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<ContainerPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }
}

impl Referable for Container {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Container { }

impl ToListMappable for Container {
    type O = ListRef<ContainerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Container_ {
    fn extract_resource_type(&self) -> String {
        "docker_container".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainer {
    pub tf_id: String,
    #[doc= "The ID of the image to back this container. The easiest way to get this value is to use the `docker_image` resource as is shown in the example."]
    pub image: PrimField<String>,
    #[doc= "The name of the container."]
    pub name: PrimField<String>,
}

impl BuildContainer {
    pub fn build(self, stack: &mut Stack) -> Container {
        let out = Container(Rc::new(Container_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attach` after provisioning.\nIf `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`."]
    pub fn attach(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.attach", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bridge` after provisioning.\nThe network bridge of the container as read from its NetworkSettings."]
    pub fn bridge(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bridge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cgroupns_mode` after provisioning.\nCgroup namespace mode to use for the container. Possible values are: `private`, `host`."]
    pub fn cgroupns_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroupns_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `command` after provisioning.\nThe command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `[\"/usr/bin/myprogram\",\"-f\",\"baz.con\"]`."]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_logs` after provisioning.\nThe logs of the container if its execution is done (`attach` must be disabled)."]
    pub fn container_logs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_logs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_read_refresh_timeout_milliseconds` after provisioning.\nThe total number of milliseconds to wait for the container to reach status 'running'"]
    pub fn container_read_refresh_timeout_milliseconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_read_refresh_timeout_milliseconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `cpu_set` after provisioning.\nA comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`."]
    pub fn cpu_set(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_set", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cpu_shares` after provisioning.\nCPU shares (relative weight) for the container."]
    pub fn cpu_shares(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_shares", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destroy_grace_seconds` after provisioning.\nIf defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop."]
    pub fn destroy_grace_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.destroy_grace_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns` after provisioning.\nDNS servers to use."]
    pub fn dns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_opts` after provisioning.\nDNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options."]
    pub fn dns_opts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_search` after provisioning.\nDNS search domains that are used when bare unqualified hostnames are used inside of the container."]
    pub fn dns_search(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domainname` after provisioning.\nDomain name of the container."]
    pub fn domainname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domainname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `entrypoint` after provisioning.\nThe command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `\"/usr/bin/myprogra\"]`."]
    pub fn entrypoint(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\nEnvironment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`"]
    pub fn env(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exit_code` after provisioning.\nThe exit code of the container if its execution is done (`must_run` must be disabled)."]
    pub fn exit_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.exit_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gpus` after provisioning.\nGPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior."]
    pub fn gpus(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_add` after provisioning.\nAdditional groups for the container user"]
    pub fn group_add(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.group_add", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nHostname of the container."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nThe ID of the image to back this container. The easiest way to get this value is to use the `docker_image` resource as is shown in the example."]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `init` after provisioning.\nConfigured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults."]
    pub fn init(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.init", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipc_mode` after provisioning.\nIPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`."]
    pub fn ipc_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_driver` after provisioning.\nThe logging driver to use for the container."]
    pub fn log_driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_driver", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_opts` after provisioning.\nKey/value pairs to use as options for the logging driver."]
    pub fn log_opts(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.log_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logs` after provisioning.\nSave the container logs (`attach` must be enabled). Defaults to `false`."]
    pub fn logs(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_retry_count` after provisioning.\nThe maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'."]
    pub fn max_retry_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retry_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\nThe memory limit for the container in MBs."]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory_swap` after provisioning.\nThe total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `terraform apply` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation."]
    pub fn memory_swap(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_swap", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `must_run` after provisioning.\nIf `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform assumes it is successful. Defaults to `true`."]
    pub fn must_run(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.must_run", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the container."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_data` after provisioning.\nThe data of the networks the container is connected to."]
    pub fn network_data(&self) -> ListRef<ContainerNetworkDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_mode` after provisioning.\nNetwork mode of the container."]
    pub fn network_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pid_mode` after provisioning.\nhe PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`."]
    pub fn pid_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `privileged` after provisioning.\nIf `true`, the container runs in privileged mode."]
    pub fn privileged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `publish_all_ports` after provisioning.\nPublish all ports of the container."]
    pub fn publish_all_ports(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publish_all_ports", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `read_only` after provisioning.\nIf `true`, the container will be started as readonly. Defaults to `false`."]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_volumes` after provisioning.\nIf `true`, it will remove anonymous volumes associated with the container. Defaults to `true`."]
    pub fn remove_volumes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_volumes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restart` after provisioning.\nThe restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`."]
    pub fn restart(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restart", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rm` after provisioning.\nIf `true`, then the container will be automatically removed when it exits. Defaults to `false`."]
    pub fn rm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runtime` after provisioning.\nRuntime to use for the container."]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_opts` after provisioning.\nList of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration."]
    pub fn security_opts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shm_size` after provisioning.\nSize of `/dev/shm` in MBs."]
    pub fn shm_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shm_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start` after provisioning.\nIf `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`."]
    pub fn start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stdin_open` after provisioning.\nIf `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`."]
    pub fn stdin_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.stdin_open", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to stop a container (default `SIGTERM`)."]
    pub fn stop_signal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stop_timeout` after provisioning.\nTimeout (in seconds) to stop a container."]
    pub fn stop_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.stop_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_opts` after provisioning.\nKey/value pairs for the storage driver options, e.g. `size`: `120G`"]
    pub fn storage_opts(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.storage_opts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sysctls` after provisioning.\nA map of kernel parameters (sysctls) to set in the container."]
    pub fn sysctls(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctls", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tmpfs` after provisioning.\nA map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options."]
    pub fn tmpfs(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tmpfs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tty` after provisioning.\nIf `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`."]
    pub fn tty(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tty", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nUser used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name."]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `userns_mode` after provisioning.\nSets the usernamespace mode for the container when usernamespace remapping option is enabled."]
    pub fn userns_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.userns_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait` after provisioning.\nIf `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`."]
    pub fn wait(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_timeout` after provisioning.\nThe timeout in seconds to wait the container to be healthy after creation. Defaults to `60`."]
    pub fn wait_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `working_dir` after provisioning.\nThe working directory for commands to run in."]
    pub fn working_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> ListRef<ContainerHealthcheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> ListRef<ContainerPortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ContainerNetworkDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_ipv6_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_ipv6_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_name: Option<PrimField<String>>,
}

impl ContainerNetworkDataEl {
    #[doc= "Set the field `gateway`.\n"]
    pub fn set_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `global_ipv6_address`.\n"]
    pub fn set_global_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.global_ipv6_address = Some(v.into());
        self
    }

    #[doc= "Set the field `global_ipv6_prefix_length`.\n"]
    pub fn set_global_ipv6_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.global_ipv6_prefix_length = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_prefix_length`.\n"]
    pub fn set_ip_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ip_prefix_length = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_gateway`.\n"]
    pub fn set_ipv6_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_gateway = Some(v.into());
        self
    }

    #[doc= "Set the field `network_name`.\n"]
    pub fn set_network_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_name = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNetworkDataEl {
    type O = BlockAssignable<ContainerNetworkDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            network_name: core::default::Default::default(),
        }
    }
}

pub struct ContainerNetworkDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNetworkDataElRef {
    fn new(shared: StackShared, base: String) -> ContainerNetworkDataElRef {
        ContainerNetworkDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNetworkDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }

    #[doc= "Get a reference to the value of field `global_ipv6_address` after provisioning.\n"]
    pub fn global_ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_ipv6_address", self.base))
    }

    #[doc= "Get a reference to the value of field `global_ipv6_prefix_length` after provisioning.\n"]
    pub fn global_ipv6_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_ipv6_prefix_length", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_prefix_length` after provisioning.\n"]
    pub fn ip_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_prefix_length", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_gateway` after provisioning.\n"]
    pub fn ipv6_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_gateway", self.base))
    }

    #[doc= "Get a reference to the value of field `network_name` after provisioning.\n"]
    pub fn network_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerCapabilitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    add: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop: Option<SetField<PrimField<String>>>,
}

impl ContainerCapabilitiesEl {
    #[doc= "Set the field `add`.\nList of linux capabilities to add."]
    pub fn set_add(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.add = Some(v.into());
        self
    }

    #[doc= "Set the field `drop`.\nList of linux capabilities to drop."]
    pub fn set_drop(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.drop = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerCapabilitiesEl {
    type O = BlockAssignable<ContainerCapabilitiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerCapabilitiesElRef {
    fn new(shared: StackShared, base: String) -> ContainerCapabilitiesElRef {
        ContainerCapabilitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerCapabilitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `add` after provisioning.\nList of linux capabilities to add."]
    pub fn add(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.add", self.base))
    }

    #[doc= "Get a reference to the value of field `drop` after provisioning.\nList of linux capabilities to drop."]
    pub fn drop(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.drop", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerDevicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_path: Option<PrimField<String>>,
    host_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<PrimField<String>>,
}

impl ContainerDevicesEl {
    #[doc= "Set the field `container_path`.\nThe path in the container where the device will be bound."]
    pub fn set_container_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_path = Some(v.into());
        self
    }

    #[doc= "Set the field `permissions`.\nThe cgroup permissions given to the container to access the device. Defaults to `rwm`."]
    pub fn set_permissions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permissions = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerDevicesEl {
    type O = BlockAssignable<ContainerDevicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerDevicesEl {
    #[doc= "The path on the host where the device is located."]
    pub host_path: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerDevicesElRef {
    fn new(shared: StackShared, base: String) -> ContainerDevicesElRef {
        ContainerDevicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerDevicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `container_path` after provisioning.\nThe path in the container where the device will be bound."]
    pub fn container_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_path", self.base))
    }

    #[doc= "Get a reference to the value of field `host_path` after provisioning.\nThe path on the host where the device is located."]
    pub fn host_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc= "Get a reference to the value of field `permissions` after provisioning.\nThe cgroup permissions given to the container to access the device. Defaults to `rwm`."]
    pub fn permissions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerHealthcheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_period: Option<PrimField<String>>,
    test: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
}

impl ContainerHealthcheckEl {
    #[doc= "Set the field `interval`.\nTime between running the check (ms|s|m|h). Defaults to `0s`."]
    pub fn set_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval = Some(v.into());
        self
    }

    #[doc= "Set the field `retries`.\nConsecutive failures needed to report unhealthy. Defaults to `0`."]
    pub fn set_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retries = Some(v.into());
        self
    }

    #[doc= "Set the field `start_period`.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn set_start_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_period = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout`.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerHealthcheckEl {
    type O = BlockAssignable<ContainerHealthcheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerHealthcheckEl {
    #[doc= "Command to run to check health. For example, to run `curl -f localhost/health` set the command to be `[\"CMD\", \"curl\", \"-f\", \"localhost/health\"]`."]
    pub test: ListField<PrimField<String>>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerHealthcheckElRef {
    fn new(shared: StackShared, base: String) -> ContainerHealthcheckElRef {
        ContainerHealthcheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerHealthcheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nTime between running the check (ms|s|m|h). Defaults to `0s`."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc= "Get a reference to the value of field `retries` after provisioning.\nConsecutive failures needed to report unhealthy. Defaults to `0`."]
    pub fn retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retries", self.base))
    }

    #[doc= "Get a reference to the value of field `start_period` after provisioning.\nStart period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`."]
    pub fn start_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_period", self.base))
    }

    #[doc= "Get a reference to the value of field `test` after provisioning.\nCommand to run to check health. For example, to run `curl -f localhost/health` set the command to be `[\"CMD\", \"curl\", \"-f\", \"localhost/health\"]`."]
    pub fn test(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.test", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nMaximum time to allow one check to run (ms|s|m|h). Defaults to `0s`."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerHostEl {
    host: PrimField<String>,
    ip: PrimField<String>,
}

impl ContainerHostEl { }

impl ToListMappable for ContainerHostEl {
    type O = BlockAssignable<ContainerHostEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerHostEl {
    #[doc= "Hostname to add"]
    pub host: PrimField<String>,
    #[doc= "IP address this hostname should resolve to."]
    pub ip: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerHostElRef {
    fn new(shared: StackShared, base: String) -> ContainerHostElRef {
        ContainerHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerHostElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nHostname to add"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nIP address this hostname should resolve to."]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerLabelsEl {
    label: PrimField<String>,
    value: PrimField<String>,
}

impl ContainerLabelsEl { }

impl ToListMappable for ContainerLabelsEl {
    type O = BlockAssignable<ContainerLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerLabelsEl {
    #[doc= "Name of the label"]
    pub label: PrimField<String>,
    #[doc= "Value of the label"]
    pub value: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerLabelsElRef {
    fn new(shared: StackShared, base: String) -> ContainerLabelsElRef {
        ContainerLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerLabelsElRef {
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

#[derive(Serialize)]
pub struct ContainerMountsElBindOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    propagation: Option<PrimField<String>>,
}

impl ContainerMountsElBindOptionsEl {
    #[doc= "Set the field `propagation`.\nA propagation mode with the value."]
    pub fn set_propagation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.propagation = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerMountsElBindOptionsEl {
    type O = BlockAssignable<ContainerMountsElBindOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerMountsElBindOptionsElRef {
    fn new(shared: StackShared, base: String) -> ContainerMountsElBindOptionsElRef {
        ContainerMountsElBindOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElBindOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `propagation` after provisioning.\nA propagation mode with the value."]
    pub fn propagation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagation", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerMountsElTmpfsOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_bytes: Option<PrimField<f64>>,
}

impl ContainerMountsElTmpfsOptionsEl {
    #[doc= "Set the field `mode`.\nThe permission mode for the tmpfs mount in an integer."]
    pub fn set_mode(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `size_bytes`.\nThe size for the tmpfs mount in bytes."]
    pub fn set_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_bytes = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerMountsElTmpfsOptionsEl {
    type O = BlockAssignable<ContainerMountsElTmpfsOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerMountsElTmpfsOptionsElRef {
    fn new(shared: StackShared, base: String) -> ContainerMountsElTmpfsOptionsElRef {
        ContainerMountsElTmpfsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElTmpfsOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe permission mode for the tmpfs mount in an integer."]
    pub fn mode(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `size_bytes` after provisioning.\nThe size for the tmpfs mount in bytes."]
    pub fn size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerMountsElVolumeOptionsElLabelsEl {
    label: PrimField<String>,
    value: PrimField<String>,
}

impl ContainerMountsElVolumeOptionsElLabelsEl { }

impl ToListMappable for ContainerMountsElVolumeOptionsElLabelsEl {
    type O = BlockAssignable<ContainerMountsElVolumeOptionsElLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerMountsElVolumeOptionsElLabelsEl {
    #[doc= "Name of the label"]
    pub label: PrimField<String>,
    #[doc= "Value of the label"]
    pub value: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerMountsElVolumeOptionsElLabelsElRef {
    fn new(shared: StackShared, base: String) -> ContainerMountsElVolumeOptionsElLabelsElRef {
        ContainerMountsElVolumeOptionsElLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElVolumeOptionsElLabelsElRef {
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
struct ContainerMountsElVolumeOptionsElDynamic {
    labels: Option<DynamicBlock<ContainerMountsElVolumeOptionsElLabelsEl>>,
}

#[derive(Serialize)]
pub struct ContainerMountsElVolumeOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_options: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_copy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<ContainerMountsElVolumeOptionsElLabelsEl>>,
    dynamic: ContainerMountsElVolumeOptionsElDynamic,
}

impl ContainerMountsElVolumeOptionsEl {
    #[doc= "Set the field `driver_name`.\nName of the driver to use to create the volume."]
    pub fn set_driver_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.driver_name = Some(v.into());
        self
    }

    #[doc= "Set the field `driver_options`.\nkey/value map of driver specific options."]
    pub fn set_driver_options(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.driver_options = Some(v.into());
        self
    }

    #[doc= "Set the field `no_copy`.\nPopulate volume with data from the target."]
    pub fn set_no_copy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_copy = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<BlockAssignable<ContainerMountsElVolumeOptionsElLabelsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.labels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ContainerMountsElVolumeOptionsEl {
    type O = BlockAssignable<ContainerMountsElVolumeOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ContainerMountsElVolumeOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerMountsElVolumeOptionsElRef {
    fn new(shared: StackShared, base: String) -> ContainerMountsElVolumeOptionsElRef {
        ContainerMountsElVolumeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElVolumeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `driver_name` after provisioning.\nName of the driver to use to create the volume."]
    pub fn driver_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.driver_name", self.base))
    }

    #[doc= "Get a reference to the value of field `driver_options` after provisioning.\nkey/value map of driver specific options."]
    pub fn driver_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.driver_options", self.base))
    }

    #[doc= "Get a reference to the value of field `no_copy` after provisioning.\nPopulate volume with data from the target."]
    pub fn no_copy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_copy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerMountsElDynamic {
    bind_options: Option<DynamicBlock<ContainerMountsElBindOptionsEl>>,
    tmpfs_options: Option<DynamicBlock<ContainerMountsElTmpfsOptionsEl>>,
    volume_options: Option<DynamicBlock<ContainerMountsElVolumeOptionsEl>>,
}

#[derive(Serialize)]
pub struct ContainerMountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    target: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bind_options: Option<Vec<ContainerMountsElBindOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tmpfs_options: Option<Vec<ContainerMountsElTmpfsOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_options: Option<Vec<ContainerMountsElVolumeOptionsEl>>,
    dynamic: ContainerMountsElDynamic,
}

impl ContainerMountsEl {
    #[doc= "Set the field `read_only`.\nWhether the mount should be read-only."]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\nMount source (e.g. a volume name, a host path)."]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc= "Set the field `bind_options`.\n"]
    pub fn set_bind_options(mut self, v: impl Into<BlockAssignable<ContainerMountsElBindOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bind_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bind_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tmpfs_options`.\n"]
    pub fn set_tmpfs_options(mut self, v: impl Into<BlockAssignable<ContainerMountsElTmpfsOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tmpfs_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tmpfs_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `volume_options`.\n"]
    pub fn set_volume_options(mut self, v: impl Into<BlockAssignable<ContainerMountsElVolumeOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.volume_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ContainerMountsEl {
    type O = BlockAssignable<ContainerMountsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerMountsEl {
    #[doc= "Container path"]
    pub target: PrimField<String>,
    #[doc= "The mount type"]
    pub type_: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct ContainerMountsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerMountsElRef {
    fn new(shared: StackShared, base: String) -> ContainerMountsElRef {
        ContainerMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read_only` after provisioning.\nWhether the mount should be read-only."]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nMount source (e.g. a volume name, a host path)."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nContainer path"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe mount type"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `bind_options` after provisioning.\n"]
    pub fn bind_options(&self) -> ListRef<ContainerMountsElBindOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bind_options", self.base))
    }

    #[doc= "Get a reference to the value of field `tmpfs_options` after provisioning.\n"]
    pub fn tmpfs_options(&self) -> ListRef<ContainerMountsElTmpfsOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tmpfs_options", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_options` after provisioning.\n"]
    pub fn volume_options(&self) -> ListRef<ContainerMountsElVolumeOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volume_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerNetworksAdvancedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
    name: PrimField<String>,
}

impl ContainerNetworksAdvancedEl {
    #[doc= "Set the field `aliases`.\nThe network aliases of the container in the specific network."]
    pub fn set_aliases(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.aliases = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv4_address`.\nThe IPV4 address of the container in the specific network."]
    pub fn set_ipv4_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv4_address = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_address`.\nThe IPV6 address of the container in the specific network."]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerNetworksAdvancedEl {
    type O = BlockAssignable<ContainerNetworksAdvancedEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerNetworksAdvancedEl {
    #[doc= "The name or id of the network to use. You can use `name` or `id` attribute from a `docker_network` resource."]
    pub name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerNetworksAdvancedElRef {
    fn new(shared: StackShared, base: String) -> ContainerNetworksAdvancedElRef {
        ContainerNetworksAdvancedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerNetworksAdvancedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `aliases` after provisioning.\nThe network aliases of the container in the specific network."]
    pub fn aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.aliases", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv4_address` after provisioning.\nThe IPV4 address of the container in the specific network."]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\nThe IPV6 address of the container in the specific network."]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name or id of the network to use. You can use `name` or `id` attribute from a `docker_network` resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerPortsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external: Option<PrimField<f64>>,
    internal: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
}

impl ContainerPortsEl {
    #[doc= "Set the field `external`.\nPort exposed out of the container. If not given a free random port `>= 32768` will be used."]
    pub fn set_external(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.external = Some(v.into());
        self
    }

    #[doc= "Set the field `ip`.\nIP address/mask that can access this port. Defaults to `0.0.0.0`."]
    pub fn set_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\nProtocol that can be used over this port. Defaults to `tcp`."]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerPortsEl {
    type O = BlockAssignable<ContainerPortsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerPortsEl {
    #[doc= "Port within the container."]
    pub internal: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerPortsElRef {
    fn new(shared: StackShared, base: String) -> ContainerPortsElRef {
        ContainerPortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerPortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `external` after provisioning.\nPort exposed out of the container. If not given a free random port `>= 32768` will be used."]
    pub fn external(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.external", self.base))
    }

    #[doc= "Get a reference to the value of field `internal` after provisioning.\nPort within the container."]
    pub fn internal(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal", self.base))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\nIP address/mask that can access this port. Defaults to `0.0.0.0`."]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\nProtocol that can be used over this port. Defaults to `tcp`."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerUlimitEl {
    hard: PrimField<f64>,
    name: PrimField<String>,
    soft: PrimField<f64>,
}

impl ContainerUlimitEl { }

impl ToListMappable for ContainerUlimitEl {
    type O = BlockAssignable<ContainerUlimitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerUlimitEl {
    #[doc= "The hard limit"]
    pub hard: PrimField<f64>,
    #[doc= "The name of the ulimit"]
    pub name: PrimField<String>,
    #[doc= "The soft limit"]
    pub soft: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerUlimitElRef {
    fn new(shared: StackShared, base: String) -> ContainerUlimitElRef {
        ContainerUlimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerUlimitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hard` after provisioning.\nThe hard limit"]
    pub fn hard(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hard", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the ulimit"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `soft` after provisioning.\nThe soft limit"]
    pub fn soft(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.soft", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerUploadEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_base64: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    executable: Option<PrimField<bool>>,
    file: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_hash: Option<PrimField<String>>,
}

impl ContainerUploadEl {
    #[doc= "Set the field `content`.\nLiteral string value to use as the object content, which will be uploaded as UTF-8-encoded text. Conflicts with `content_base64` & `source`"]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }

    #[doc= "Set the field `content_base64`.\nBase64-encoded data that will be decoded and uploaded as raw bytes for the object content. This allows safely uploading non-UTF8 binary data, but is recommended only for larger binary content such as the result of the `base64encode` interpolation function. See [here](https://github.com/terraform-providers/terraform-provider-docker/issues/48#issuecomment-374174588) for the reason. Conflicts with `content` & `source`"]
    pub fn set_content_base64(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_base64 = Some(v.into());
        self
    }

    #[doc= "Set the field `executable`.\nIf `true`, the file will be uploaded with user executable permission. Defaults to `false`."]
    pub fn set_executable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.executable = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\nA filename that references a file which will be uploaded as the object content. This allows for large file uploads that do not get stored in state. Conflicts with `content` & `content_base64`"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc= "Set the field `source_hash`.\nIf using `source`, this will force an update if the file content has updated but the filename has not. "]
    pub fn set_source_hash(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_hash = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerUploadEl {
    type O = BlockAssignable<ContainerUploadEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerUploadEl {
    #[doc= "Path to the file in the container where is upload goes to"]
    pub file: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerUploadElRef {
    fn new(shared: StackShared, base: String) -> ContainerUploadElRef {
        ContainerUploadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerUploadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nLiteral string value to use as the object content, which will be uploaded as UTF-8-encoded text. Conflicts with `content_base64` & `source`"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc= "Get a reference to the value of field `content_base64` after provisioning.\nBase64-encoded data that will be decoded and uploaded as raw bytes for the object content. This allows safely uploading non-UTF8 binary data, but is recommended only for larger binary content such as the result of the `base64encode` interpolation function. See [here](https://github.com/terraform-providers/terraform-provider-docker/issues/48#issuecomment-374174588) for the reason. Conflicts with `content` & `source`"]
    pub fn content_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.base))
    }

    #[doc= "Get a reference to the value of field `executable` after provisioning.\nIf `true`, the file will be uploaded with user executable permission. Defaults to `false`."]
    pub fn executable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.executable", self.base))
    }

    #[doc= "Get a reference to the value of field `file` after provisioning.\nPath to the file in the container where is upload goes to"]
    pub fn file(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nA filename that references a file which will be uploaded as the object content. This allows for large file uploads that do not get stored in state. Conflicts with `content` & `content_base64`"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc= "Get a reference to the value of field `source_hash` after provisioning.\nIf using `source`, this will force an update if the file content has updated but the filename has not. "]
    pub fn source_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_hash", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_container: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_name: Option<PrimField<String>>,
}

impl ContainerVolumesEl {
    #[doc= "Set the field `container_path`.\nThe path in the container where the volume will be mounted."]
    pub fn set_container_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_path = Some(v.into());
        self
    }

    #[doc= "Set the field `from_container`.\nThe container where the volume is coming from."]
    pub fn set_from_container(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.from_container = Some(v.into());
        self
    }

    #[doc= "Set the field `host_path`.\nThe path on the host where the volume is coming from."]
    pub fn set_host_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_path = Some(v.into());
        self
    }

    #[doc= "Set the field `read_only`.\nIf `true`, this volume will be readonly. Defaults to `false`."]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_name`.\nThe name of the docker volume which should be mounted."]
    pub fn set_volume_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_name = Some(v.into());
        self
    }
}

impl ToListMappable for ContainerVolumesEl {
    type O = BlockAssignable<ContainerVolumesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ContainerVolumesElRef {
    fn new(shared: StackShared, base: String) -> ContainerVolumesElRef {
        ContainerVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `container_path` after provisioning.\nThe path in the container where the volume will be mounted."]
    pub fn container_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_path", self.base))
    }

    #[doc= "Get a reference to the value of field `from_container` after provisioning.\nThe container where the volume is coming from."]
    pub fn from_container(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_container", self.base))
    }

    #[doc= "Get a reference to the value of field `host_path` after provisioning.\nThe path on the host where the volume is coming from."]
    pub fn host_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc= "Get a reference to the value of field `read_only` after provisioning.\nIf `true`, this volume will be readonly. Defaults to `false`."]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_name` after provisioning.\nThe name of the docker volume which should be mounted."]
    pub fn volume_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerDynamic {
    capabilities: Option<DynamicBlock<ContainerCapabilitiesEl>>,
    devices: Option<DynamicBlock<ContainerDevicesEl>>,
    healthcheck: Option<DynamicBlock<ContainerHealthcheckEl>>,
    host: Option<DynamicBlock<ContainerHostEl>>,
    labels: Option<DynamicBlock<ContainerLabelsEl>>,
    mounts: Option<DynamicBlock<ContainerMountsEl>>,
    networks_advanced: Option<DynamicBlock<ContainerNetworksAdvancedEl>>,
    ports: Option<DynamicBlock<ContainerPortsEl>>,
    ulimit: Option<DynamicBlock<ContainerUlimitEl>>,
    upload: Option<DynamicBlock<ContainerUploadEl>>,
    volumes: Option<DynamicBlock<ContainerVolumesEl>>,
}
