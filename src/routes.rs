use crate::handlers::bookmark_handlers::{
    create_bookmark, delete_bookmark, get_bookmark, list_bookmarks, update_bookmark,
};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/bookmarks")
                .service(list_bookmarks)
                .service(get_bookmark)
                .service(create_bookmark)
                .service(update_bookmark)
                .service(delete_bookmark),
        ),
    );
}
