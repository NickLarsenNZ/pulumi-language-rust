// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// server interface

pub trait LanguageRuntime {
    fn get_required_plugins(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::language::GetRequiredPluginsRequest>, resp: ::grpc::ServerResponseUnarySink<super::language::GetRequiredPluginsResponse>) -> ::grpc::Result<()>;

    fn run(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::language::RunRequest>, resp: ::grpc::ServerResponseUnarySink<super::language::RunResponse>) -> ::grpc::Result<()>;

    fn get_plugin_info(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::empty::Empty>, resp: ::grpc::ServerResponseUnarySink<super::plugin::PluginInfo>) -> ::grpc::Result<()>;
}

// client

pub struct LanguageRuntimeClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for LanguageRuntimeClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        LanguageRuntimeClient {
            grpc_client: grpc_client,
        }
    }
}

impl LanguageRuntimeClient {
    pub fn get_required_plugins(&self, o: ::grpc::RequestOptions, req: super::language::GetRequiredPluginsRequest) -> ::grpc::SingleResponse<super::language::GetRequiredPluginsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/pulumirpc.LanguageRuntime/GetRequiredPlugins"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn run(&self, o: ::grpc::RequestOptions, req: super::language::RunRequest) -> ::grpc::SingleResponse<super::language::RunResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/pulumirpc.LanguageRuntime/Run"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_plugin_info(&self, o: ::grpc::RequestOptions, req: super::empty::Empty) -> ::grpc::SingleResponse<super::plugin::PluginInfo> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/pulumirpc.LanguageRuntime/GetPluginInfo"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct LanguageRuntimeServer;


impl LanguageRuntimeServer {
    pub fn new_service_def<H : LanguageRuntime + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/pulumirpc.LanguageRuntime",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/pulumirpc.LanguageRuntime/GetRequiredPlugins"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_required_plugins(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/pulumirpc.LanguageRuntime/Run"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).run(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/pulumirpc.LanguageRuntime/GetPluginInfo"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_plugin_info(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
