use crate::*;

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::{Service, ServiceExt};
    use http_body_util::BodyExt;
    use super::*;

    async fn drop_db(connection: &Connection) {
        connection.execute("DROP TABLE IF EXISTS orders;", []).unwrap();
    }

    async fn app() -> Router {
        let connection: Connection = Connection::open("test_database.db").unwrap();
        drop_db(&connection).await;
        init_db(&connection).await;
        let connection: Arc<Mutex<Connection>> = Arc::new(Mutex::new(connection));
        init_app(&connection).await
    }

    #[tokio::test]
    async fn get_empty_orders_test() {
        let app = app().await;

        let response = app
            .oneshot(
                Request::builder()
                .uri("/orders")
                .body(Body::empty())
                .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: [Order;0] = serde_json::from_slice(&body).unwrap();

        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn post_get_order_test() {
        let mut app = app().await.into_service();

        let mut order = Order::default();
        order.order_uid = String::from("1");

        // post order
        let request = Request::builder()
            .method("POST")
            .uri("/orders")
            .header("content-type", "application/json")
            .body(Body::new(serde_json::to_string(&order).unwrap()))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // get order
        let request = Request::builder()
            .uri("/orders/1")
            .body(Body::empty())
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Order = serde_json::from_slice(&body).unwrap();

        assert_eq!(order, body);
    }

    #[tokio::test]
    async fn get_existing_order_test() {
        let mut app = app().await.into_service();

        let mut order = Order::default();
        order.order_uid = String::from("1");
        let content = serde_json::to_string(&order).unwrap();

        // post order
        let request = Request::builder()
            .method("POST")
            .uri("/orders")
            .header("content-type", "application/json")
            .body(Body::new(content.clone()))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // post order again
        let request = Request::builder()
            .method("POST")
            .uri("/orders")
            .header("content-type", "application/json")
            .body(Body::new(content))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_ne!(response.status(), StatusCode::OK);
    }
}