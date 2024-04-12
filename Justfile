# use with https://github.com/casey/just

c := "/Cargo.toml"
h := "heartbeat-"
s := "server"
cl := "client"
q := '"'

alias bs := build_server
alias bc := build_client
alias bb := build_both

alias rs := run_server
alias rc := run_client

# requires watchexec https://github.com/watchexec/watchexec
alias ws := watch_server
alias wc := watch_client

# intended use:
#   just build server
#   just build client
build target:
    cargo build --manifest-path={{h}}{{target}}{{c}}

# intended use:
#   just run server
#   just run client
run target:
    cargo run --manifest-path={{h}}{{target}}{{c}}

# intended use:
#   just watch server
#   just watch client
#   this will just build lol i have no idea how to make cargo watch
#   or watchexec work correctly with cargo run
watch target:
    watchexec -w {{h}}{{target}}/src -- just build {{target}}

build_server: (build s)
build_client: (build cl)
build_both: 
    just bs
    just bc

run_server: (run s)
run_client: (run cl)

# requires watchexec https://github.com/watchexec/watchexec
watch_server: (watch s)
watch_client: (watch cl)

# this can't be good practice
commitpush reason:
    git add .
    git commit -m {{q}}{{reason}}{{q}}
    git push origin dev