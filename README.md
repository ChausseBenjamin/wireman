![](img/logo.jpg)



## What is StellaRPC?

StellaRPC is a gRPC client that runs in the terminal! Simply put your proto files in a config and be ready to make gRPC request

## Demo

![](img/screen2.png)


## Yank as `grpcurl` request

stellaRPC supports copying the the request data as a `grpcurl` command. This makes collaborating with peers a breeze.
Just type ctrl+y on the Request tab and the request is saved to the clipboard.
```
grpcurl -d @ -import-path /User/stellarpc/test_utils -proto grpc_simple/timekeeper.proto -plaintext localhost:50051 proto.TimeKeeper.GetNameOfMonth <<EOM
{
  "number": 0
}
EOM
```

## Features:
- [x] List Services & Methods
- [x] Request can be edited
- [x] TLS support
- [x] Unary gRPC client calls
- [ ] Streaming gRPC client calls
- [x] Metadata specification
- [x] Server address specification
- [ ] Request History
- [ ] Extended message description
- [x] Defaults of repeated/nested fields
- [x] Yank/Paste from clipboard
- [x] Yank request as grpcurl command
