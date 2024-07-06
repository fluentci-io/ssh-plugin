use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn setup(_args: String) -> FnResult<String> {
    let stdout = helpers::setup()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn x(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["drone-ssh", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn scp(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["drone-scp", &args])?
        .stdout()?;
    Ok(stdout)
}
