# heartbeat-client

The heartbeat-client expects a config.toml (see [config.toml.template](config.toml.template)) adjacent to the executable, if you do not have one the program will give you the option of generating one.

Sample config.toml:
```
key = "13212312123"
mqtt_host = "127.0.0.1"
mqtt_port = 1883
mqtt_topic = "test/topic"
```