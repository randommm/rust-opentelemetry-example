An example of setting up OpenTelemetry tracing with Rust.

## Usage

Start the Jaeger Docker container:

```bash
docker run --rm --name jaeger -e COLLECTOR_ZIPKIN_HOST_PORT:9411 -e COLLECTOR_OTLP_ENABLED:true -p 6831:6831/udp -p 6832:6832/udp -p 5778:5778 -p 16686:16686 -p 4317:4317 -p 4318:4318 -p 14250:14250 -p 14268:14268 -p 14269:14269 -p 9411:9411 jaegertracing/all-in-one
```

Run the Rust application:

```bash
cargo run
```

View the traces in the Jaeger UI:

```bash
open http://127.0.0.1:16686
```
