use std::{sync::{Arc, Mutex}, vec};
use serde::{Deserialize, Serialize};
use axum::{extract::{Json, Path}, response::IntoResponse, routing::{get, Router}, http::StatusCode, Extension};
use rusqlite::{Connection, Row};
use serde_json::json;

mod tests;

#[tokio::main]
async fn main() {
    let connection: Connection = Connection::open("database.db").unwrap();
    init_db(&connection).await;
    println!("The database has been successfully initialized");
    let connection: Arc<Mutex<Connection>> = Arc::new(Mutex::new(connection));

    let app: Router = init_app(&connection).await;

    println!("The application listens for requests at localhost:3000");
    init_server(app, "0.0.0.0:3000").await;
}

// init
async fn init_db(connection: &Connection) {
    connection.execute(CREATE_ORDERS, []).unwrap();
}

async fn init_app(connection: &Arc<Mutex<Connection>>) -> Router {
    Router::new()
        .route("/orders", get(get_orders).post(post_order))
        .route("/orders/:order_uid", get(get_order))
        .layer(Extension(Arc::clone(connection)))
}

async fn init_server(app: Router, addr: &str) {
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .tcp_nodelay(true)
        .await
        .unwrap();
}

// handlers
enum AppError {
    Unknown(String),
    OrderAlreadyExists(String),
    OrderNotExists(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            Self::Unknown(err) => (StatusCode::BAD_REQUEST, err),
            Self::OrderAlreadyExists(order_uid) => (StatusCode::BAD_REQUEST, format!("Order with order_uid={} already exists", order_uid)),
            Self::OrderNotExists(order_uid) => (StatusCode::BAD_REQUEST, format!("Order with order_uid={} does not exist", order_uid)),
        };
        return (status, msg).into_response();
    }
}

async fn get_orders(Extension(connection): Extension<Arc<Mutex<Connection>>>) -> Result<Json<Vec<Order>>, AppError> {
    let connection = connection.lock().map_err(|err|{
        AppError::Unknown(err.to_string())
    })?;
    let mut stmt = connection.prepare("SELECT * FROM orders").map_err(|err|{
        AppError::Unknown(err.to_string())
    })?;
    
    let orders: Vec<Order> = stmt.query_map([], |row| {
        <&Row<'_> as TryInto<Order>>::try_into(row)
    }).map_err(|err|{
        AppError::Unknown(err.to_string())
    })?.map(|value|{
        value.unwrap()
    }).collect();
    
    return Ok(Json(orders));
}

async fn get_order(Path(order_uid): Path<String>, Extension(connection): Extension<Arc<Mutex<Connection>>>) -> Result<Json<Order>, AppError> {
    let connection = connection.lock().map_err(|err|{
        AppError::Unknown(err.to_string())
    })?;

    let query = &format!("SELECT * FROM orders WHERE order_uid = ('{}');", &order_uid);
    let mut stmt = connection.prepare(query).map_err(|_|{
        AppError::OrderNotExists(order_uid.clone())
    })?;

    if stmt.column_count() <= 0 {
        return Err(AppError::OrderNotExists(order_uid.clone()));
    }

    let order = stmt.query_row([], |row| {
        <&Row<'_> as TryInto<Order>>::try_into(row)
    }).map_err(|_|{
        AppError::OrderNotExists(order_uid.clone())
    })?;
        
    return Ok(Json(order));
}

async fn post_order(Extension(connection): Extension<Arc<Mutex<Connection>>>, Json(order): Json<Order>) -> Result<(), AppError> {
    let connection = connection.lock().map_err(|err|{
        AppError::Unknown(err.to_string())
    })?;

    let query = &format!("SELECT * FROM orders WHERE order_uid = ('{}');", order.order_uid);
    let mut stmt = connection.prepare(query).map_err(|err|{
        AppError::Unknown(err.to_string())
    })?;

    let existing_order = stmt.query_row([], |row| {
        <&Row<'_> as TryInto<Order>>::try_into(row)
    });

    if let Ok(_) = existing_order {
        return Err(AppError::OrderAlreadyExists(order.order_uid));
    }

    connection.execute(
        r#"INSERT INTO orders (order_uid, track_number, entry, delivery, payment, items, locale, internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)"#,
        [
                order.order_uid,
                order.track_number,
                order.entry, 
                serde_json::to_string(&order.delivery).unwrap(), 
                serde_json::to_string(&order.payment).unwrap(), 
                serde_json::to_string(&order.items).unwrap(),
                order.locale, 
                order.internal_signature, 
                order.customer_id, 
                order.delivery_service, 
                order.shardkey, 
                format!("{}", order.sm_id), 
                order.date_created, 
                order.oof_shard
                ]
    ).map_err(|err|{
        AppError::Unknown(err.to_string())
    })?;

    return Ok(());    
}

// impls
impl TryFrom<&Row<'_>> for Order {
    type Error = rusqlite::Error;
    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        let delivery: String = value.get(4)?;
        let delivery: Delivery = serde_json::from_str(json!(delivery).as_str().unwrap()).unwrap();

        let payment: String = value.get(5)?;
        let payment: Payment = serde_json::from_str(json!(payment).as_str().unwrap()).unwrap();

        let items: String = value.get(6)?;
        let items: Vec<Item> = serde_json::from_str(json!(items).as_str().unwrap()).unwrap();

        let date_created: String = value.get(13)?;

        Ok(Order {
            order_uid: value.get(1)?,
            track_number: value.get(2)?, 
            entry: value.get(3)?, 
            delivery: delivery, 
            payment: payment, 
            items: items, 
            locale: value.get(7)?, 
            internal_signature: value.get(8)?, 
            customer_id: value.get(9)?, 
            delivery_service: value.get(10)?, 
            shardkey: value.get(11)?, 
            sm_id: value.get(12)?, 
            date_created: date_created, 
            oof_shard: value.get(14)?
        })
    }
}

// models
#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: u32,
    payment_dt: i64,
    bank: String,
    delivery_cost: u32,
    goods_total: u32,
    custom_fee: u32,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
struct Item {
    chrt_id: i64,
    track_number: String,
    price: u32,
    rid: String,
    name: String,
    sale: u32,
    size: String,
    total_price: u32,
    nm_id: i64,
    brand: String,
    status: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
}

impl Default for Order {
    fn default() -> Self {
        Order {
            order_uid: String::default(),
            track_number: String::default(), 
            entry: String::default(), 
            delivery: Delivery::default(), 
            payment: Payment::default(), 
            items: vec![Item::default().clone(); 2], 
            locale: String::default(), 
            internal_signature: String::default(), 
            customer_id: String::default(), 
            delivery_service: String::default(), 
            shardkey: String::default(), 
            sm_id: i32::default(), 
            date_created: String::new(),
            oof_shard: String::default() 
        }
    }
}

// consts
const CREATE_ORDERS: &str = r#"
    CREATE TABLE IF NOT EXISTS orders (
        [id] SERIAL PRIMARY KEY,
        [order_uid] VARCHAR(255) NOT NULL,
        [track_number] VARCHAR(255) NOT NULL,
        [entry] VARCHAR(255) NOT NULL,
        [delivery] JSON NOT NULL,
        [payment] JSON NOT NULL,
        [items] JSON NOT NULL,
        [locale] VARCHAR(255),
        [internal_signature] VARCHAR(255),
        [customer_id] VARCHAR(255),
        [delivery_service] VARCHAR(255),
        [shardkey] VARCHAR(255),
        [sm_id] INTEGER,
        [date_created] TIMESTAMP,
        [oof_shard] VARCHAR(255)
    );
"#;