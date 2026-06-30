fn main() {
    let protos = [
        "../../proto/tracker-cluster/monitor.proto",
        "../../proto/tracker/tracker-service.proto",
    ];
    for proto in &protos {
        tonic_prost_build::compile_protos(proto)
            .unwrap_or_else(|e| panic!("Failed to compile proto {:?}", e));
    }
}