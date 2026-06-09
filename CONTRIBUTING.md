## Build
#### Install Toolchains
- Rust 1.92
- Dioxus 0.7.9

https://dioxuslabs.com/learn/0.7/getting_started/

### Vs Code Setup
#### Install Recommend Extensions
Paste @recommended in Search Extensions (Ctrl+Shift+X)
- rust-analyzer
- Dioxus
- Code spell check

### Final Checks
Run: `dev.env.check.sh` or Vs Code task (Ctrl+Shift+X -> Tasks: Run task): `Check::Env`

This should pass with no errors.

### Run locally with hotreloading
Vs Code task: `Serve::localhost`
```shell
dx serve
```

### Release Build
Vs Code task: `Build::Release`
```shell
dx build --web --release
```
Output will be in: `target/dx/openltomanager/release/`