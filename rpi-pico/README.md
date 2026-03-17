## Introduction

Projects for raspberry pi pico platform

## Creating new project

1. Copy the started project code from [folder](./projects/project00_blinky/)

2. Make sure the project folder name and project name in project's Cargo.toml file should match. Otherwise debug session would fail.
   See example

- [Project folder](./projects/project00_blinky/)
- [Project's Cargo.toml](./projects/project00_blinky/Cargo.toml)

```toml
[package]
edition = "2021"
name = "project00_blinky"
version = "0.1.0"
license = "MIT OR Apache-2.0"
```

3. After creating project folder execute below commands to create symbolic to `.vscode` and `.cargo` folder from root to project folder

```sh
ln -s $PICO_ROOT_DIR/.vscode .vscode
ln -s $PICO_ROOT_DIR/.cargo .cargo
```

## Projects

1.  **[Blinky](./projects/project00_blinky/):** Classic embedded hello world program.
2.  **[Buttons](./projects/project01_buttons/):** Example of using buttons
