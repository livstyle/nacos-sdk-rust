use crate::common::executor;
use crate::common::remote::generate_request_id;
use crate::common::remote::grpc::message::GrpcMessageData;
use crate::naming::message::response::InstanceResponse;
use crate::naming::redo::CallBack;
use crate::naming::{message::request::InstanceRequest, redo::AutomaticRequest};

impl AutomaticRequest for InstanceRequest {
    fn run(
        &self,
        nacos_grpc_client: std::sync::Arc<crate::common::remote::grpc::NacosGrpcClient>,
        call_back: CallBack,
    ) {
        let mut request = self.clone();
        request.request_id = Some(generate_request_id());

        executor::spawn(async move {
            let ret = nacos_grpc_client
                .unary_call_async::<InstanceRequest, InstanceResponse>(request)
                .await;
            if let Err(e) = ret {
                call_back(Err(e));
            } else {
                call_back(Ok(()));
            }
        });
    }

    fn name(&self) -> String {
        let namespace = self.namespace.as_deref().unwrap_or("");
        let service_name = self.service_name.as_deref().unwrap_or("");
        let group_name = self.group_name.as_deref().unwrap_or("");

        let request_name = format!(
            "{}@@{}@@{}@@{}",
            namespace,
            group_name,
            service_name,
            InstanceRequest::identity()
        );
        request_name
    }
}