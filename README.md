# Ssh Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/ssh)](https://pkg.fluentci.io/ssh)
[![ci](https://github.com/fluentci-io/ssh-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/ssh-plugin/actions/workflows/ci.yml)

CI Plugin to execute commands on a remote host through SSH. See [Drone SSH](https://github.com/appleboy/drone-ssh) and [Drone SCP](https://github.com/appleboy/drone-scp) for more information.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm ssh setup
```

## Functions

| Name   | Description                               |
| ------ | ----------------------------------------- |
| setup  | Install drone-ssh and drone-scp           |
| x      | Execute a command on a remote host        |
| scp    | Copy files to a remote host               |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/ssh@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: ssh
    args: |
      setup
 - name: Verify installation
   run: |
      type drone-ssh
      type drone-scp
```
