![f3f86624-a5b0-4c60-a44b-4dc1a2ce25a0](https://github.com/rainshowerLabs/sothis/assets/55022497/a5e5dda2-875c-4d88-88bd-88d5f945854d)

# `sothis`

Sothis is a tool for replaying historical state on a local ***anvil/hardhat*** testnet node. 

## Support and discussion

Join the [Rainshower Labs discord server](https://discord.gg/Cs3h397gkz) to discuss sothis and get help.

## Usage

Sothis has optional arguments that are not listed in their respective mode sections that might prove useful. Please study the help section below. You can view it any time by running `sothis --help`.

```
Tool for replaying historical transactions. Designed to be used with anvil or hardhat.

Usage: sothis [OPTIONS] --source_rpc <source_rpc>...

Options:
  -s, --source_rpc <source_rpc>...
          HTTP JSON-RPC of the node we're querying data from
  -r, --replay_rpc <replay_rpc>...
          HTTP JSON-RPC of the node we're replaying data to
  -m, --mode <mode>...
          Choose between live, historic, track, or fast_track [default: historic]
  -b, --terminal_block <terminal_block>...
          Block we're replaying until
      --exit_on_tx_fail [<exit_on_tx_fail>...]
          Exit the program if a transaction fails
  -t, --block_listen_time <block_listen_time>...
          Time in ms to check for new blocks. [default: 500]
      --entropy_threshold <entropy_threshold>...
          Set the percentage of failed transactions to trigger a warning [default: 0.07]
  -d, --replay_delay <replay_delay>...
          Default delay for block replay in ms [default: 0]
      --send_as_unsigned [<send_as_unsigned>...]
          Exit the program if a transaction fails
      --no_setup [<no_setup>...]
          Start replaying immediately.
  -c, --contract_address <contract_address>...
          Address of the contract we're tracking storage.
  -l, --storage_slot <storage_slot>...
          Storage slot for the variable we're tracking
  -0, --origin_block <origin_block>...
          First block sothis will look at.
  -p, --path <path>...
          Path to file we're writing to [default: .]
  -f, --filename <filename>...
          Name of the file. [default: ]
  -h, --help
          Print help
  -V, --version
          Print version

```

Sothis currently has 4 modes. Live, historic, track, and fast track.   

### Historic

Historic mode is the default way to use sothis. Its used to replay state to a local node forked to a deep historical block.

#### Usage

- `-m historic`(optinal): Used to denote we are replaying in live mode.
- `--source_rpc`: RPC of the node we are getting blocks from.
- `--replay_rpc`: RPC of the node were sending blocks to.
- `--terminal_block`: Final block sothis will replay.

To stop replaying, terminate the process via Ctrl+C or however else you preffer.

```
sothis --source_rpc {ARCHIVE_NODE} --replay_rpc http://localhost:8545 -m historic --terminal_block 9000022
```

### Live

Live mode is designed to be used with a forked local node, with its tip near the head block. It replays the latest blocks as they come to your forked node.

#### Usage

- `-m live`: Used to denote we are replaying in live mode.
- `--source_rpc`: RPC of the node we are getting blocks from.
- `--replay_rpc`: RPC of the node were sending blocks to.

To stop replaying, terminate the process via Ctrl+C or however else you preffer.

```
sothis --source_rpc {ARCHIVE_NODE} --replay_rpc http://localhost:8545 -m live
```

### Track

The tracking mode is used to track the change in value of a storage slot for a contract, that needs to be updated live. It can be used on a live production network, as well as in conjuntion with sothis (keep in mind that you can use the `--block_listen_time` so tracking doesn't lag behind!) . If you are testing on a local network, you can launch another instance of sothis to track the change of a slot on a replay node.   

The result is saved to a JSON file that looks like this:
```json
{
  "address":"0x1c479675ad559DC151F6Ec7ed3FbF8ceE79582B6",
	"storage_slot":"0x0",
	"state_changes":[
		{"block_number":"0x10b7bbc","value":"0x00000000000000000000000000000000000000000000000000000000000e2b18"}
	]
}
```

#### Usage

- `--mode track`: Used to denote we are using the tracking mode.
- `--source_rpc`: RPC of the node we are getting data from.
- `--contract_address`: Address of the contract we are reading storage from.
- `--storage_slot`: The storage slot of the contract.
- `--terminal_block`(optional): Final block sothis will track. If not specified, sothis will track until terminated.
- `--filename`(optional): Name of our output file. The default filename is formatted as: `address-{}-slot-{}-timestamp-{}.json`.
- `--path`(optional): Path to our output file. The default path is the current directory.

Once you are done tracking the slot, terminate the process via a `SIGTERM` or a `SIGINT` (ctrl-c), which will terminate execution and write the file. Keep in mind that sothis will check once per new block if you tried to terminate it. If no new block are produced on the source_rpc, sothis will not terminate and nothing will be written if you force close it.

`sothis --mode track --source_rpc http://localhost:8545 --contract_address 0x1c479675ad559DC151F6Ec7ed3FbF8ceE79582B6 --storage_slot 0 --filename siuuu.json --path ~/Desktop
`

### Fast track

The fast track mode is used to track the change for a *historic* storage slot. It cannot be used to get a live view of it. The source_rpc must be an archive node for this mode to perform optimally. This results in a speedup of as much as ~10000% compared to the regular tracking mode. This is the recommended mode to use if you do not have a local node.

#### Usage

- `--mode fast_track`: Used to denote we are using the tracking mode.
- `--source_rpc`: RPC of the node we are getting data from.
- `--contract_address`: Address of the contract we are reading storage from.
- `--storage_slot`: The storage slot of the contract.
- `--origin_block`: The block from which we start tracking.
- `--terminal_block`(optional): Final block sothis will track. If not specified, sothis will track until terminated.
- `--filename`(optional): Name of our output file. The default filename is formatted as: `address-{}-slot-{}-timestamp-{}.json`.
- `--path`(optional): Path to our output file. The default path is the current directory.

Once you are done tracking the slot, terminate the process via a `SIGTERM` or a `SIGINT` (ctrl-c), which will terminate execution and write the file. Keep in mind that sothis will check once per new block if you tried to terminate it. If no new block are produced on the source_rpc, sothis will not terminate and nothing will be written if you force close it.
<!-- easter egg contract -->
`sothis --mode track --source_rpc http://localhost:8545 --contract_address 0x910cbd523d972eb0a6f4cae4618ad62622b39dbf --storage_slot 3 --filename siuuu.json --path ~/Desktop
`

## Installation

Sothis is a rust crate. You can install it with cargo:
`cargo install sothis`

## FAQ

###  Why is sothis so slow?

Sothis uses a lot of JSON-RPC calls. This may cause your RPC provider to throttle you. It's recommended to use your own local node.       
If using `anvil` make sure you add the `--cups {REALL_HIGH_VALUE}` arg so anvil doesn't throttle itself.

### I have a problem with sothis. Can devs do something?

Yes! Make a github issue detailing your problem.

### Why the name?

Sothis is known as the creator and God of Fódlan in Fire Emblem: Thee Houses. She has the ability to rewind time at will.

## todo

- ??????
