# heartbeat
Software I'm writing to track usage statistics from my devices/accounts.  
Consists of [heartbeat-client](heartbeat-client) and [heartbeat-server](heartbeat-server). Both have individual READMEs.

uses [Just](https://github.com/casey/just) to handle build stuff:  
```
just run {target}
just build {target}
just watch {target} # not currently functional, prefer `just build {target}; just run {target};`
```