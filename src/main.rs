use axum::{
    extract::{Json, Path, State},
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

#[tokio::main]
async fn main() {
    let shared_state = SharedState::default();

    // Build our application by composing routes
    let app = Router::new()
        .route(
            "/customers",
            get(get_customers).post_service(create_customer.with_state(Arc::clone(&shared_state))),
        )
        .route("/customer/:id", get(get_customer))
        .with_state(Arc::clone(&shared_state));

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    customers: HashMap<usize, Customer>,
}

async fn get_customer(
    Path(key): Path<usize>,
    State(state): State<SharedState>,
) -> Result<Json<Customer>, StatusCode> {
    let customers = &state.read().unwrap().customers;

    if let Some(value) = customers.get(&key) {
        Ok(Json(value.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn create_customer(
    State(state): State<SharedState>,
    new_customer: Json<NewCustomer>,
) -> impl IntoResponse {
    let customers = &mut state.write().unwrap().customers;
    let key = customers.len();
    let customer = Customer {
        id: key,
        first_name: new_customer.first_name.clone(),
        last_name: new_customer.last_name.clone(),
        email: new_customer.email.clone(),
        associated_ethereum_addresses: new_customer.associated_ethereum_addresses.clone(),
    };
    customers.insert(key, customer);
    StatusCode::CREATED
}

async fn get_customers(State(state): State<SharedState>) -> Json<Vec<Customer>> {
    let customers = &state.read().unwrap().customers;
    Json(customers.values().cloned().collect())
}

#[derive(Debug, Deserialize)]
struct NewCustomer {
    first_name: String,
    last_name: String,
    email: String,
    associated_ethereum_addresses: Vec<String>,
}
#[derive(Debug, Serialize, Clone)]
struct Customer {
    id: usize,
    first_name: String,
    last_name: String,
    email: String,
    associated_ethereum_addresses: Vec<String>,
}