curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "say_hello", "id":1 }' 127.0.0.1:3030
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "Slot.List", "id":2 }' 127.0.0.1:3030
