# CLI interface to Fantom Foundation EVM.

## Usage

**host** and **port** specify node API end point of the EVM servicing user requests.

```
$funrpc --help
funrpc 0.1.0
Fantom Foundation <contact@fantom.foundation>
Lachesis node CLI interface

USAGE:
    funrpc [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -d, --debug      Activate debug mode
        --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)

OPTIONS:
    -h, --host <hostname>    node API hostname/address [default: localhost]
    -p, --port <port>        Node API port number [default: 8080]

SUBCOMMANDS:
    account     get any account
    accounts    get controlled accounts
    help        Prints this message or the help of the given subcommand(s)
    rawtx       sends raw signed transaction
    tx          sends transaction or get transaction status
```

## *accounts* command

This command shows controlled accounts available.

```
$funrpc accounts --help
funrpc-accounts 0.1.0
Fantom Foundation <contact@fantom.foundation>
get controlled accounts

USAGE:
    funrpc accounts

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

**Example:**
```
$funrpc --host testnet0 accounts
```


## *account* command

This command shows any account available.

```
$funrpc account --help
funrpc-account 0.1.0
Fantom Foundation <contact@fantom.foundation>
get any account

USAGE:
    funrpc account --number <number>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --number <number>
```

**Example:**
```
$funrpc --host testnet0 account --number 0xfd9AB87eCEdC912A63f5B8fa3b8d7667d33Fd981
{"address":"0xfd9AB87eCEdC912A63f5B8fa3b8d7667d33Fd981","balance":4900000000000000000000000000,"nonce":0}
```

## *tx* command

This command sends transaction from controlled accounds and get transaction receipt (for  any transaction sent using *tx* or *rawtx* command).

```
$funrpc tx --help
funrpc-tx 0.1.0
Fantom Foundation <contact@fantom.foundation>
sends transaction or get transaction status

USAGE:
    funrpc tx <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    d       Sends transaction JSON
    h       request status of transaction
    help    Prints this message or the help of the given subcommand(s)
```

### *d* subcommand to send a transaction from a controlled account:
```
$funrpc tx d --help
funrpc-tx-d 0.1.0
Fantom Foundation <contact@fantom.foundation>
Sends transaction JSON

USAGE:
    funrpc tx d --from <from> --to <to> --value <value>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --from <from>      account from
    -t, --to <to>          account to
    -v, --value <value>    value in transaction
```
**Example:**
```
$funrpc --host testnet0 tx d --from 0x629007eb99ff5c3539ada8a5800847eacfc25727 --to 0xe32e14de8b81d8d3aedacb1868619c74a68feab0 --value 0x1
{"txHash":"0xeeeed34877502baa305442e3a72df094cfbb0b928a7c53447745ff35d50020bf"}
```

### *h* subcommand get transaction receipt by transaction hash
```
$funrpc tx h --help
funrpc-tx-h 0.1.0
Fantom Foundation <contact@fantom.foundation>
request status of transaction

USAGE:
    funrpc tx h <tx_hash>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <tx_hash>    Hexadecimal transaction hash
```
**Example:**
```
$funrpc --host testnet0 tx h 0xeeeed34877502baa305442e3a72df094cfbb0b928a7c53447745ff35d50020bf
{"to":"0xe32e14de8b81d8d3aedacb1868619c74a68feab0","root":"0xc8f90911c9280651a0cd84116826d31773e902e48cb9a15b7bb1e7a6abc850c5","gasUsed":"0x5208","from":"0x629007eb99ff5c3539ada8a5800847eacfc25727","transactionHash":"0xeeeed34877502baa305442e3a72df094cfbb0b928a7c53447745ff35d50020bf","logs":[],"cumulativeGasUsed":"0x5208","contractAddress":null,"logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"}
```

## *rawtx* command

This command sends a raw signed transaction
```
$funrpc rawtx --help
funrpc-rawtx 0.1.0
Fantom Foundation <contact@fantom.foundation>
sends raw signed transaction

USAGE:
    funrpc rawtx -d <doc>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <doc>        raw transaction
```

**Example:**
```
$funrpc --host testnet0 rawtx -d 0xf8628080830f424094564686380e267d1572ee409368e1d42081562a8e8201f48026a022b4f68bfbd4f4c309524ebdbf4bac858e0ad65fd06108c934b45a6da88b92f7a046433c388997fd7b02eb7128f4d2401ef2d10d574c42edf15875a43ee51a1993
{"txHash":"0x5496489c606d74ad7435568393fa2c4619e64497267f80864109277631aa849d"}
```