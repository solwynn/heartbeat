# Heartbeat

### Todo

-  Server
    - [ ] Authentication handler
        - Server should allow the creation of "modules" who have unique IDs, and unique authentication keys. Maybe unique ID not necessary? can probably just fetch module based on authentication key in DB
    - [ ] DB
        - Should store the latest payload somewhere just for data retention when the server goes down 
    - [x] Read config (config-rs)
    - [ ] Admin panel (2FA?) 

- Client
    - [ ] Authentication handler
    - [x] Read config (config-rs)

- Project
    - [x] Justfile
    - [ ] Config templates ?
        - by default I think I'm going to look for configs adjacent to the binary and allow paths as a runtime flag