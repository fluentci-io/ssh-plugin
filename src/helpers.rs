use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    let arch = dag().get_arch()?;
    let os = dag().get_os()?;

    let arch = match arch.as_str() {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => arch.as_str(),
    };

    let os = match os.as_str() {
        "linux" => "linux",
        "macos" => "darwin",
        _ => os.as_str(),
    };

    let drone_ssh_version = dag().get_env("DRONE_SSH_VERSION")?;
    let drone_scp_version = dag().get_env("DRONE_SCP_VERSION")?;

    let drone_ssh_version = match drone_ssh_version.as_str() {
        "" => "1.7.6",
        _ => drone_ssh_version.as_str(),
    };

    let drone_scp_version = match drone_scp_version.as_str() {
        "" => "1.6.14",
        _ => drone_scp_version.as_str(),
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "sh",
            "-c",
            &format!("type drone-ssh > /dev/null || pkgx wget https://github.com/appleboy/drone-ssh/releases/download/v{}/drone-ssh-{}-{}-{}", drone_ssh_version, drone_ssh_version, os, arch),
        ])?
        .with_exec(vec![
            "sh",
            "-c",
            &format!("type drone-scp > /dev/null || pkgx wget https://github.com/appleboy/drone-scp/releases/download/v{}/drone-scp-{}-{}-{}",drone_scp_version, drone_scp_version, os, arch),
        ])?
        .stdout()?;
    Ok(stdout)
}
