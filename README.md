![sothis_logo](https://github.com/makemake-kbo/sothis/assets/55022497/c4508232-71e2-42c8-aaec-aa35c668983b)

# `sothis`

Sothis is a tool for replaying historical state on a local ***anvil/hardhat(soon)*** testnet node. 

## Usage

Sothis currently has 2 modes. Live and historic. 

### Historic

Historic mode is the default way to use sothis. Its used to replay state to a local node forked to a deep historical block.

#### Usage

- `--source_rpc`: RPC of the node we are getting blocks from.
- `--replay_rpc`: RPC of the node were sending blocks to.
- `-m historic`(optinal): Used to denote we are replaying in live mode.
- `--terminal_block`: Final block sothis will replay.

To stop replaying, terminate the process via Ctrl+C or however else you preffer.

```
sothis --source_rpc {ARCHIVE_NODE} --replay_rpc http://localhost:8545 -m historic --terminal_block 9000022
```

### Live

Live mode is designed to be used with a forked local node, with its tip near the head block. It replays the latest blocks as they come to your forked node.

#### Usage

- `--source_rpc`: RPC of the node we are getting blocks from.
- `--replay_rpc`: RPC of the node were sending blocks to.
- `-m live`: Used to denote we are replaying in live mode.

To stop replaying, terminate the process via Ctrl+C or however else you preffer.

```
sothis --source_rpc {ARCHIVE_NODE} --replay_rpc http://localhost:8545 -m live
```

## Installation

Sothis is a rust crate. You can install it with cargo:
`cargo install sothis`

## FAQ

###  Why is sothis so slow?

Its most likely your RPC provider/s. If using `anvil` make sure you add the `--cups {REALL_HIGH_VALUE}` arg so it doesnt get throttled.

### I have a problem with sothis. Can devs do something?

Yes! Make a github issue detailing your problem.

### Why the name?

Sothis is known as the creator and God of Fódlan in Fire Emblem: Thee Houses. She has the ability to rewind time at will.

### todo

- track historical state var changes
- ??????

