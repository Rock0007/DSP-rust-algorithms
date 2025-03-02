use tonic::transport::Server;
use shift_planner::shift_planner_server::{ShiftPlanner, ShiftPlannerServer};
use shift_planner::{ShiftRequest, ShiftResponse};
use algorithms::csp::{ShiftInput, rebalance_shift};
use algorithms::greedy::assign_shift; // Add this import

mod shift_planner {
    tonic::include_proto!("shift_planner");
}

mod algorithms;

#[derive(Debug, Default)]
pub struct ShiftPlannerService;

#[tonic::async_trait]
impl ShiftPlanner for ShiftPlannerService {
    async fn rebalance_shift(
        &self,
        request: tonic::Request<ShiftRequest>,
    ) -> Result<tonic::Response<ShiftResponse>, tonic::Status> {
        let input_data = request.into_inner().data;
        let input: ShiftInput = serde_json::from_str(&input_data)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;

        let output = rebalance_shift(input);
        let response = ShiftResponse {
            data: serde_json::to_string(&output).unwrap(),
        };
        Ok(tonic::Response::new(response))
    }

    async fn assign_shift(
        &self,
        request: tonic::Request<ShiftRequest>,
    ) -> Result<tonic::Response<ShiftResponse>, tonic::Status> {
        let input_data = request.into_inner().data;
        let input: ShiftInput = serde_json::from_str(&input_data)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;

        let output = assign_shift(input);
        let response = ShiftResponse {
            data: serde_json::to_string(&output).unwrap(),
        };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let service = ShiftPlannerService::default();
    println!("Rust Algorithm Service running on {}", addr);
    Server::builder()
        .add_service(ShiftPlannerServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}