# Transforming Substrate Project Configurations

## Table of Contents

1. [Fork the Substrate Cumulus Node Parachain Template Repository](#1-fork-the-substrate-cumulus-node-parachain-template-repository)
2. [Define Chain Specification Configuration](#2-define-chain-specification-configuration)
3. [Replace Parachain Template Runtime](#3-replace-parachain-template-runtime)
4. [Move Octopus Application Pallets](#4-move-octopus-application-pallets)

---

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


