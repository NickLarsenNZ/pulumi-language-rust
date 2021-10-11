pub mod language;
pub mod language_grpc;
pub mod plugin;

pub mod empty {
    pub use protobuf::well_known_types::Empty;
}
