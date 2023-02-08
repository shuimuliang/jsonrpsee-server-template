# jsonrpsee-server-template
json rpc http server based on jsonrpsee, with PostgreSQL integrated

# usage
## server
```shell
cargo run --bin jsonrpsee-server-template
```

## client
```shell
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "say_hello", "id":123 }' 127.0.0.1:3030
```
