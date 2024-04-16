# Heartbeat

### Todo

-  Server
    - [ ] Authentication handler
        - Server should allow the creation of "modules" who have unique IDs, and unique authentication keys. Maybe unique ID not necessary? can probably just fetch module based on authentication key in DB
    - [ ] Read config (config-rs)
    - [ ] Soft exit if no config 
    - [ ] Admin panel (2FA?) 

- Client
    - [ ] Authentication handler
    - [x] Read config (config-rs)
    - [ ] Soft exit if no config 

- Project
    - [x] Justfile
    - [ ] Config templates ?
        - by default I think I'm going to look for configs adjacent to the binary and allow paths as a runtime flag