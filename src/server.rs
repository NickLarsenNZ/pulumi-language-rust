mod gen;
use gen::language::*;
use gen::language_grpc::*;

//use crate::gen::language_grpc::LanguageRuntimeServer;

struct PulumiRust;

impl LanguageRuntime for PulumiRust {
    fn get_required_plugins(
        &self,
        o: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<GetRequiredPluginsRequest>,
        resp: grpc::ServerResponseUnarySink<GetRequiredPluginsResponse>,
    ) -> grpc::Result<()> {
        todo!()
    }

    fn run(
        &self,
        o: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<RunRequest>,
        resp: grpc::ServerResponseUnarySink<RunResponse>,
    ) -> grpc::Result<()> {
        todo!()
    }

    fn get_plugin_info(
        &self,
        o: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<gen::empty::Empty>,
        resp: grpc::ServerResponseUnarySink<gen::plugin::PluginInfo>,
    ) -> grpc::Result<()> {
        todo!()
    }
}

fn main() {
    let port = 50051;

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port);
    server.add_service(LanguageRuntimeServer::new_service_def(PulumiRust));

    server.build().expect("Well that didn't work");

    println!("service started on port {}", port);

    loop {
        std::thread::park();
    }
}
