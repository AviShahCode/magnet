# Magnet

**Magnet** is a personal self-hosted server for storage, compute, and remote access.

It is built to replace dependence on third-party cloud services with a system you fully control.

## Run server

After cloning the repository, follow instructions in queries/setup.sql, then edit the postgres url and run:

```bash
source serve.sh
```

Make sure to add the static/config.toml file and an empty drive folder in the build directory.

## Run client

After creating a virtual environment and installing maturin, run:

```bash
maturin develop
```

in client, this will install the client extension for python. The python API is mentioned in client/readme.md.

## Planned Features
- Private, Google-Drive-like file storage
- File sync and access control
- Remote code execution
- Modular, extensible services

## Status
Early development. Expect breakage.

## Philosophy
Own the hardware. Own the data. Own the execution.
