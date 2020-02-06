pub mod todo_controller;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    todo_controller::routes()
}
