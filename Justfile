# use with https://github.com/casey/just

c := "/Cargo.toml"
h := "heartbeat-"
s := "server"
cl := "client"
q := '"'

# target = server/client
build target:
    cargo build --manifest-path={{h}}{{target}}{{c}}

# target = server/client
run target:
    cargo run --manifest-path={{h}}{{target}}{{c}}

# does not work lol
watch target:
    cargo watch -C {{h}}{{s}} -x build && run  

# this can't be good practice
commitpush reason:
    git add .
    git commit -m {{q}}{{reason}}{{q}}
    git push origin dev