# Resolver of problem [helloworld-rust-project](https://github.com/massbitprotocol/helloworld-rust-project)

## Description

In example, we use [Kademlia](https://en.wikipedia.org/wiki/Kademlia) for distribute p2p in memory data, and mDNS for auto discovery nodes based on tis [example](https://github.com/libp2p/rust-libp2p/blob/master/examples/distributed-key-value-store.rs) of [rust-libp2p](https://github.com/libp2p/rust-libp2p).

For HTTP server api, we use [jsonrpc](https://github.com/paritytech/jsonrpc/tree/master/http) with JSON-RPC 2.0.

## How to test

### Start multi nodes (open 5 terminals for simple) by run command
```
cargo run 3001
cargo run 3002
cargo run 3003
cargo run 3004
cargo run 3005
```

where is 3001, 3002, 3003, 3004, 3005 is http server api in JSON-RPC 2.0.

### Set value in node

Try to set pokemon with properties 
```
{"name":"test", "color":"green", "eye_num":2, "nose_num":2, "mouth_num":5}
```
to node that listen 3001

```
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "put", "params": [{"name":"test", "color":"green", "eye_num":2, "nose_num":2, "mouth_num":5}], "id":1 }' 127.0.0.1:3001

```

You can update or create new pokemon with other name in others port.

### Get value in nodes

Try to get pokemon with name is "test" in node 3001 

```
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "get", "params": ["test"], "id":1 }' 127.0.0.1:3001
```

You can test with others node to verify synchronous of data.
