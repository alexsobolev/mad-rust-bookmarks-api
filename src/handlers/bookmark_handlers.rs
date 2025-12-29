use crate::{
    error::AppError,
    extractors::ValidatedJson,
    models::{CreateBookmarkRequest, SearchParams, UpdateBookmarkRequest},
    state::AppState,
};
use actix_web::{HttpResponse, delete, get, post, put, web};

#[get("")]
pub async fn list_bookmarks(
    state: web::Data<AppState>,
    query: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let bookmarks = state
        .bookmark_service
        .list(
            query.tag.as_deref(),
            query.unread_only,
            query.page,
            query.size,
        )
        .await?;
    Ok(HttpResponse::Ok().json(bookmarks))
}

#[get("/{id}")]
pub async fn get_bookmark(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let bookmark = state.bookmark_service.get(&id).await?;
    Ok(HttpResponse::Ok().json(bookmark))
}

#[post("")]
pub async fn create_bookmark(
    state: web::Data<AppState>,
    ValidatedJson(request): ValidatedJson<CreateBookmarkRequest>,
) -> Result<HttpResponse, AppError> {
    let created = state.bookmark_service.create(request).await?;
    Ok(HttpResponse::Created().json(created))
}

#[put("/{id}")]
pub async fn update_bookmark(
    state: web::Data<AppState>,
    path: web::Path<String>,
    ValidatedJson(request): ValidatedJson<UpdateBookmarkRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let updated = state.bookmark_service.update(&id, request).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/{id}")]
pub async fn delete_bookmark(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    state.bookmark_service.delete(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}
