use crate::model::todo_model::TodoModel;
use crate::service::todo_service;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    index().or(save()).or(update()).or(delete()).or(detail())
}

pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todo")
        .and(warp::get())
        .and_then(todo_service::index)
}
pub fn save() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todo")
        .and(warp::post())
        .and(json_body())
        .and_then(todo_service::save)
}
pub fn update() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todo" / u64)
        .and(warp::put())
        .and(json_body())
        .and_then(todo_service::update)
}
pub fn delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todo" / u64)
        .and(warp::delete())
        .and_then(todo_service::delete)
}
pub fn detail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todo" / u64)
        .and(warp::get())
        .and_then(todo_service::detail)
}

fn json_body() -> impl Filter<Extract = (TodoModel,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
