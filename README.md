<div align="center">
<img src="https://avatars.githubusercontent.com/u/80524516?s=200&v=4">
</div>
<br>
<br>

<div align="Center">
<h1>Myriad Node</h1>
<h2>It's Your Turn to Own Your Web</h2>
Starting with Myriad.Social, we are creating a platform where social app, metaverse and messenger seamlessly integrate, together and with further applications. As a user, a content creator or a builder, Myriad is designed to be yours.
<br>
<br>

[![Substrate](https://img.shields.io/badge/Substrate-3.0.0-brightgreen?logo=Parity%20Substrate)](https://substrate.io)
[![Medium](https://img.shields.io/badge/Medium-Myriad-brightgreen?logo=medium)](https://medium.com/@myriadsocial.blog)
</div>

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Rust Setup](#rust-setup)
3. [Single-Node Development Chain](#single-node-development-chain)
4. [Run in Docker](#run-in-docker)
5. [Migrating Octopus Appchain to Rococo Parachain](#migrating-octopus-appchain-to-rococo-parachain)
6. [Fork the Substrate Cumulus Node Parachain Template Repository](#1-fork-the-substrate-cumulus-node-parachain-template-repository)
7. [Define Chain Specification Configuration](#2-define-chain-specification-configuration)
8. [Replace Parachain Template Runtime](#3-replace-parachain-template-runtime)
9. [Move Octopus Application Pallets](#4-move-octopus-application-pallets)

---

## Getting Started

Follow these steps to get started with the Node

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/debug/myriad \
--base-path .local \
--dev \
--alice \
--collator \
--force-authoring \
-- \
--execution wasm \
--dev
```

Purge the development chain's state:

```bash
./target/debug/myriad \
purge-chain \
--base-path .local \
--dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/debug/myriad \
./target/debug/myriad \
--base-path .local \
--dev \
--alice \
--collator \
--force-authoring \
-lruntime=debug \
-- \
--execution wasm \
--dev
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./.maintain/docker/create-network.sh
```

```bash
./.maintain/docker/start-docker-compose.sh
```

---

## Migrating Octopus Appchain to Rococo Parachain

### 1. Fork the Substrate Cumulus Node Parachain Template Repository

**Step 1:** Begin by forking a new repository based on the [Substrate Cumulus Node Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template).

* **Action Required:** Click the "Fork" button on the GitHub repository page to create a copy under your account.

---

### 2. Define Chain Specification Configuration

**Step 2:** In order to configure your Substrate project, you need to define the Chain Specification Configuration. This configuration encompasses essential details about your blockchain. These details include:

- **ID:** A unique identifier for your blockchain.
- **Name:** The name of your blockchain.
- **Chain Type:** The type of blockchain (e.g., Parachain, Relay Chain).
- **Chain Genesis:** Information about the initial state of your blockchain, including privileged accounts, collator accounts, pre-funded accounts, and the parachain ID.
- **Bootnodes:** Addresses of nodes that can be used as entry points into your blockchain.
- **Telemetry:** Telemetry server details, if applicable.
- **Protocol ID:** The identifier for the network protocol.
- **Fork ID:** ID for specifying forks in the blockchain.
- **Properties:** Miscellaneous properties or settings.
- **Extension:** Any additional extensions or customizations.

It's important to configure the Chain Specification for the following contexts:
- **Local Chain:** Configuration for a local development environment.
- **Development Chain:** Configuration for a development environment.
- **Rococo Chain:** Configuration for integration with the Rococo testnet.

---

### 3. Replace Parachain Template Runtime

**Step 3:** Replace the default runtime of the Substrate Cumulus Node Parachain Template, which is known as `parachain_template_runtime`, with the runtime of the Octopus application that you intend to migrate to the parachain.

* **Action Required:** Replace the runtime code files as needed to integrate the Octopus application runtime.

---

### 4. Move Octopus Application Pallets

**Step 4:** Move the Octopus application's pallet code into the new parachain repository. Ensure that all pallets are placed within the `pallets` folder of the repository.

* **Action Required:** Organize the Octopus application's pallet code files within the `pallets` directory of your new repository.

---

These steps provide a comprehensive guide for transforming Substrate project configurations. Ensure that you follow each step carefully to successfully configure your Substrate-based parachain with the Octopus application runtime.

Feel free to reach out if you have any questions or need further assistance during this process.


