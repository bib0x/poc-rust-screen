# PoC Rust: Remote screen

## Overview

Just a PoC to spwan a screen on a remote machine using OpenSSH and 
execute a command from the screen session.

The behavior can be reproduce from the Bash command below:
```
$ ssh myserver "screen -S rusty -d -m && screen -r rusty -X stuff $'ls -l\n'"
```

## Usage

```
poc-rscreen 

USAGE:
    poc-rscreen [OPTIONS] --server <server> --cmd <cmd>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cmd <cmd>          Command to execute in screen
    -n, --name <name>        Screen session name [default: rusty]
    -s, --server <server>    Sets the targeted server hostname
```
 
## Example

```
$ cat ~/.ssh/config
Host myserver
user bib0x
hostname 10.0.2.13
IdentityFile ~/.ssh/myserver

$ poc-rscreen -s myserver --cmd "ls -l" 
Status: exit code: 0
Status: exit code: 0

$ ssh myserver
$ screen -ls
There are screens on:
	4559.rusty	(06/12/2021 06:33:22 AM)	(Detached)
...
$ screen -x rusty

# Command "ls -l" has been executed in the screen session
$ ls -l
total 36916
...
```
