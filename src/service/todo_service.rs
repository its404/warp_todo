use crate::model::todo_model::TodoModel;

pub async fn save(model: TodoModel) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Create TODO: {}", model.name))
}

pub async fn index() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("List TODO"))
}

pub async fn update(id: u64, model: TodoModel) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Update TODO: {}, {}", id, model.name))
}

pub async fn delete(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Delete TODO: {}", id))
}

pub async fn detail(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Show TODO Detail: {}", id))
}
