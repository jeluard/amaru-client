A set of API / CLI to interact with a remote [amaru](https://github.com/pragma-org/amaru) node.
Currently allows to track OTEL traces (over OTLP/gRPC) and metrics (over OTLP/HTTP).

```shell
> cargo install --git https://github.com/jeluard/amaru-client.git

> amaru-client
{"traceId":"11a103abf6a7bee035539064336b5f8d","spanId":"a432079657a87aa8","traceState":"","parentSpanId":"30ecdcb106c5350b","flags":257,"name":"chain_sync.decode_header","kind":1,"startTimeUnixNano":"1768496589662642000","endTimeUnixNano":"1768496589662649000","attributes":[{"key":"code.file.path","value":{"stringValue":"crates/amaru-consensus/src/consensus/stages/receive_header.rs"}},{"key":"code.module.name","value":{"stringValue":"amaru_consensus::consensus::stages::receive_header"}},{"key":"code.line.number","value":{"intValue":"106"}},{"key":"thread.id","value":{"intValue":"11"}},{"key":"thread.name","value":{"stringValue":"tokio-runtime-worker"}},{"key":"level","value":{"stringValue":"TRACE"}},{"key":"target","value":{"stringValue":"amaru_consensus::consensus::stages::receive_header"}},{"key":"point.slot","value":{"stringValue":"58957554"}},{"key":"point.hash","value":{"stringValue":"cce647cb2387b2a4a9eefa03bde469507a8fa80175354771fa6692515cfbd9a6"}},{"key":"busy_ns","value":{"intValue":"5417"}},{"key":"idle_ns","value":{"intValue":"2375"}}],"droppedAttributesCount":0,"events":[],"droppedEventsCount":0,"links":[],"droppedLinksCount":0,"status":{"message":"","code":0}}
{"name":"cardano_node_metrics_density_real","description":"chain density over the last k blocks or since genesis, whichever is shorter","unit":"real","metadata":[],"gauge":{"dataPoints":[{"attributes":[],"startTimeUnixNano":"1768565542875727000","timeUnixNano":"1768566198612988000","exemplars":[],"flags":0,"asDouble":0.036006001000166696}]}}

> # alternatively
> amaru-client --traces-address [::1]:4317 --metrics-address [::1]:4318
> amaru-client traces --address [::1]:4317
> amaru-client metrics --address [::1]:4318
```