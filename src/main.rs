mod controller;
mod model;
mod service;

#[tokio::main]
async fn main() {
    let routes = controller::routes();
    warp::serve(routes).run(([127, 0, 0, 1], 8010)).await;
}
