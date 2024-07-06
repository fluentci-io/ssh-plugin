use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    let arch = dag().get_arch()?;
    let os = dag().get_os()?;
    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;
    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

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
            &format!("type drone-ssh > /dev/null || chmod a+x drone-ssh-*")])?
        .with_exec(vec![
            "sh",
            "-c",
            "type drone-ssh > /dev/null || mv drone-ssh-* $HOME./local/bin/drone-ssh",
        ])?
        .with_exec(vec![
            "sh",
            "-c",
            &format!("type drone-scp > /dev/null || pkgx wget https://github.com/appleboy/drone-scp/releases/download/v{}/drone-scp-{}-{}-{}",drone_scp_version, drone_scp_version, os, arch),
        ])?
        .with_exec(vec![
            "sh",
            "-c",
            &format!("type drone-scp > /dev/null || chmod a+x drone-scp-*")])?
        .with_exec(vec![
            "sh",
            "-c",
            "type drone-scp > /dev/null || mv drone-scp-* $HOME/.local/bin/drone-scp",
        ])?
        .stdout()?;
    Ok(stdout)
}
