# Immunefi Bug Bounties (`ibb`)

**Effortlessly find and filter data on Immunefi Bug Bounty Programs**  
Is like [jq](https://github.com/jqlang/jq) for Immunefi REST API. Search, filter and map structured data about bug bounty programs with easy. Pipe the output into other cli tools.

## Usage

### Basic Commands

List an array with all programs.
```sh
> ibb

["layerzero", "stargate", "sky", "sparklend", "reserve"...
```

Show details for a specific program
> ibb <program_name>

```sh
> ibb moonbeamnetwork

#...a JSON object with all details will be printed
```

Filter by fields (e.g., assets, URLs, documentation)
> ibb <program_name> <field> [<nested_field> ...]

```sh
> ibb moonbeamnetwork assets

[
  {
    "addedAt": "2022-04-20T20:27:06.665Z",
    "description": "Blockchain - Moonbeam",
    "id": "5Uv4I8GXpfWeu9SX6b5Jhz",
    "isPrimacyOfImpact": null,
    "isSafeHarbor": null,
    "type": "blockchain_dlt",
    "url": "https://github.com/PureStake/moonbeam"
  },
  #... more JSON objects here
```

```sh
> ibb moonbeamnetwork assets url

[
  "https://github.com/PureStake/moonbeam",
  "https://github.com/PureStake/nimbus",
  "https://github.com/PureStake/crowdloan-rewards",
  "https://github.com/paritytech/frontier",
  "https://github.com/ethereum-lists/chains/blob/master/_data/chains/eip155-1285.json",
  "https://github.com/ethereum-lists/chains/blob/master/_data/chains/eip155-1287.json",
  "https://github.com/ethereum-lists/chains/blob/master/_data/chains/eip155-1287.json",
  "https://apps.moonbeam.network",
  "https://crowdloan.moonbeam.foundation/",
  "https://moonbeam.network/",
  "https://moonbeam.foundation/"
]
```

Find recursively any field returned by Immunefi's REST API for a specific Bug Bounty Program
> ibb [any_program] [any_field]

```sh
> ibb 0x programDocumentations

[
  {
    "createdAt": "2024-10-25T14:41:27.620Z",
    "description": "Program Documentation",
    "id": 51,
    "programId": 2249,
    "title": "0x Documentation",
    "updatedAt": "2024-10-25T14:41:27.620Z",
    "url": "https://0x.org/docs/"
  },
  {
    "createdAt": "2024-10-25T14:43:28.812Z",
    "description": "Further Documentation",
    "id": 52,
    "programId": 2249,
    "title": "0x Documentation",
    "updatedAt": "2024-10-25T14:43:28.812Z",
    "url": "https://0x.org/docs/developer-resources/settler"
  }
]
```

Filter the output as much as desired by adding nested fields
> ibb [any_program] [any_field] [nestedfield_1] [nestedfield_2]

```sh
> ibb moonbeamnetwork bounty impacts title

[
  "DoS of greater than 10% but less than 30% of validator or miner nodes and does not shut down the network",
  "An exploit underpricing transaction fees relative to computation time",
  "Framing sensitive pages leading to financial loss (ClickJacking)",
  "Any impact involving a publicly released CVE without a working PoC",
  "An attack causing an unintended chain split (Network partition)",
  "An attack causing transient consensus failures; which recover without manual intervention",
  "Spoofing content on the target application (Persistent)",
  "Users Confidential information disclosure such as Email",
  "Privilege escalation to access unauthorized functionalities",
  "Taking down the application/website",
  "An attack causing high compute consumption by validator/mining nodes",
  "Attacks that are limited to thin clients and cannot be exploited against full nodes",
  "DoS of greater than 30% of validator or miner nodes and does not shut down the network",
  "An attack causing an RPC API crash",
  "Changing details of other users without direct financial impact (CSRF)",
  "Third-Party API keys leakage that demonstrates loss of funds or modification on the website",
  "Redirecting users to malicious websites (open redirect)",
  "A bug in layer 1 blockchain code resulting in unintended smart contract behavior (no concrete funds at risk)",
  "An attack triggering the network not being able to confirm new transactions (Total network shutdown)",
  "An attack causing an unintended permanent chain split requiring hard fork (Network partition requiring hard fork)",
  "An attack causing direct loss of funds",
  "An attack causing permanent freezing of funds (fix requires hardfork)",
  "An attack causing the minting/creation of network utility tokens (MOVR/GLMR) outside of the normal, on-chain inflation mechanism",
  "Ability to execute system commands",
  "Extract Sensitive data/files from the server such as /etc/passwd",
  "Stealing User Cookies",
  "Signing transactions for other users",
  "Redirection of user deposits and withdrawals",
  "Wallet interaction modification resulting in financial loss",
  "Subdomain takeover resulting in financial loss (applicable for subdomains with addresses published)",
  "Direct theft of user funds",
  "Tampering with transactions submitted to the user’s wallet",
  "Submitting malicious transactions to an already-connected wallet"
]
```

## Installation

### Cargo (crates.io)

If you already have a Rust environment set up, you can use the `cargo install` command:

```sh
cargo install ibb
```

Cargo will build the binary and place it in your `CARGO_INSTALL_ROOT`. For more details on installation location see the [cargo book](https://doc.rust-lang.org/cargo/commands/cargo-install.html#description).

### Cargo (git)

If you already have a Rust environment set up, you can use the `cargo install` command in your local clone of the repo:
```sh
git clone https://github.com/infosec-us-team/ibb
cd ibb
cargo install --path .
```


