use crate::interceptors::auth_interceptor::AuthInterceptor;
use crate::services::connection::connection_service::proto_connection_service_server::ProtoConnectionServiceServer;
use crate::services::connection::ConnectionService;
use crate::services::query::custom_query::custom_query_service::proto_custom_query_service_server::ProtoCustomQueryServiceServer;
use crate::services::query::custom_query::CustomQueryService;
use crate::services::query::insert::insert_service::proto_row_insert_service_server::ProtoRowInsertServiceServer;
use crate::services::query::insert::InsertService;
use crate::services::query::query_data::query_service::proto_query_service_server::ProtoQueryServiceServer;
use crate::services::query::query_data::QueryService;
use schemajs_internal::manager::InternalManager;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tonic::transport::Server;
use tonic_middleware::InterceptorFor;

pub struct GrpcServer {
    db_manager: Arc<InternalManager>,
    ip: SocketAddr,
}

pub struct GrpcServerArgs {
    pub db_manager: Arc<InternalManager>,
    pub ip: Option<String>,
}

impl GrpcServer {
    pub fn new(args: GrpcServerArgs) -> Self {
        let default_ip = args.db_manager.get_config().grpc.host.clone();
        Self {
            db_manager: args.db_manager,
            ip: args.ip.unwrap_or_else(|| default_ip).parse().unwrap(),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let curr_db = self.db_manager.clone();

        let connection_service =
            ProtoConnectionServiceServer::new(ConnectionService::new(curr_db.clone()));

        let insert_service = ProtoRowInsertServiceServer::new(InsertService::new(curr_db.clone()));

        let query_service = ProtoQueryServiceServer::new(QueryService::new(curr_db.clone()));

        let custom_query_service =
            ProtoCustomQueryServiceServer::new(CustomQueryService::new(curr_db.clone()));

        let _ = Server::builder()
            .add_service(InterceptorFor::new(
                insert_service,
                AuthInterceptor {
                    engine: curr_db.clone(),
                },
            ))
            .add_service(InterceptorFor::new(
                query_service,
                AuthInterceptor {
                    engine: curr_db.clone(),
                },
            ))
            .add_service(InterceptorFor::new(
                custom_query_service,
                AuthInterceptor {
                    engine: curr_db.clone(),
                },
            ))
            .add_service(connection_service)
            .serve(self.ip.clone())
            .await?;

        Ok(())
    }
}
