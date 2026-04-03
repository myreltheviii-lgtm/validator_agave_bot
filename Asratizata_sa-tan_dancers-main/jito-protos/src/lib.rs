pub mod proto {
    pub mod auth {
        tonic::include_proto!("auth");
    }

    pub mod block_engine {
        tonic::include_proto!("block_engine");
    }

    pub mod bundle {
        tonic::include_proto!("bundle");
    }

    pub mod packet {
        tonic::include_proto!("packet");
    }

    pub mod relayer {
        tonic::include_proto!("relayer");
    }

    pub mod shared {
        tonic::include_proto!("shared");
    }

    pub mod bam_api {
        tonic::include_proto!("bam_api");
    }

    pub mod bam_types {
        tonic::include_proto!("bam_types");
    }

    // shredstream.proto is compiled by build.rs from jito-protos/protos/shredstream.proto.
    // The generated module contains ShredstreamProxyClient (the tonic gRPC client),
    // SubscribeEntriesRequest (sent to initiate the entry stream), and
    // EntryNotification (the proto message received per batch, carrying slot,
    // parent_slot, and the wincode-serialized entry bytes passed to
    // SpeculativeSlotExecutor::execute). Without this declaration, the module
    // is compiled but invisible to the Rust module tree, causing
    // "could not find `shredstream` in `jito_protos::proto`".
    pub mod shredstream {
        tonic::include_proto!("shredstream");
    }
}
