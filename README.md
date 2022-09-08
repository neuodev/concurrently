# Concurrently

Run multiple commands concurrently

<p align="center">
    <img src="./logo.png"/>
</p>

# Preview

<p align="center">
    <img src="./preview.gif"/>
</p>

# Usage

#### Basic usage

`concurrently "command1" "command2"`

```bash
concurrently "ping -c 100 google.com" "ping -c 100 example.com"
```

Output

```bash
ping: 64 bytes from 93.184.216.34: icmp_seq=0 ttl=45 time=214.182 ms
ping: PING google.com (172.217.171.238): 56 data bytes
```

## With names

```bash
concurrently -n Google,Example "ping -c 100 google.com" "ping -c 100 example.com"
```

Output

```bash
Google: PING google.com (172.217.171.238): 56 data bytes
Google: 64 bytes from 172.217.171.238: icmp_seq=0 ttl=109 time=87.086 ms
Example: PING example.com (93.184.216.34): 56 data bytes
Example: 64 bytes from 93.184.216.34: icmp_seq=0 ttl=45 time=229.462 ms
```

```bash
concurrently 1.0.0
Ahmed Ibrahim
Run multiple commands concurrently

USAGE:
    concurrently.exe [OPTIONS] <commands>...

ARGS:
    <commands>...    Set multiple commands to concurrently

OPTIONS:
    -h, --help        Print help information
    -n <names>        A comma separated values represent a name fore each running process
    -s <sep>          Separator of the processe names [default: ,]
    -V, --version     Print version information
```
