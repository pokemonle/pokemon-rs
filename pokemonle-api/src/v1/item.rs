use crate::error::Result;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::extract::{Path, Query, State};
use pokemonle_lib::{
    database::{entity::prelude::*, r#trait::LocalizedResourceHandler},
    types::prelude::*,
};

use super::AppState;

async fn list_items_with_pagination(
    State(state): State<AppState>,
    Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
    Query(Language { lang }): Query<Language>,
    Query(SearchQuery { q }): Query<SearchQuery>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<Items, ItemNames> = &state.pool;
    Result::from(handler.list_with_pagination(page, per_page, lang, q).await)
}

async fn get_item_by_id(
    State(state): State<AppState>,
    Path(ResourceId { id }): Path<ResourceId>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<Items, ItemNames> = &state.pool;
    Result::from(handler.get_by_id(id, lang).await)
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list_items_with_pagination, |op| op.tag("item")),
        )
        .api_route("/{id}", get_with(get_item_by_id, |op| op.tag("item")))
}
