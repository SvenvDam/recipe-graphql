use warp::{self, Rejection, Reply};
use warp::Filter;
use warp::filters::BoxedFilter;

use crate::db::{Context, PostgresPool};
use crate::graphql::schema;
use crate::handlers::handle_login;
use crate::models::http::Credentials;
use crate::repository::UserRepository;

pub fn get_routes(pool: PostgresPool) -> impl Filter<Extract=impl Reply, Error=Rejection> {
    let graphiql = warp::get2()
        .and(warp::path("graphiql"))
        .and(warp::path::end())
        .and(juniper_warp::graphiql_filter("/graphql"))
        .boxed();

    let graphql = warp::post2()
        .and(warp::path("graphql"))
        .and(warp::path::end())
        .and(juniper_warp::make_graphql_filter(schema(), get_context(pool.clone())))
        .boxed();

    let auth = warp::path("login")
        .and(warp::path::end())
        .and(get_context(pool.clone()))
        .and(warp::query::<Credentials>())
        .map(|ctx: Context, creds: Credentials| {
            let res = UserRepository::try_login(
                &ctx.pool.get().unwrap(),
                &creds.username,
                &creds.password,
            );

            (res.clone(), creds.username.clone())
        })
        .and_then(handle_login);

    auth
        .or(graphql.or(graphiql))
        .with(warp::log("server"))
}

fn get_context(pool: PostgresPool) -> BoxedFilter<(Context, )> {
    warp::any()
        .and(warp::cookie::optional("User-Session-Token"))
        .map(move |token_cookie: Option<String>| {
            let (user, token) = match token_cookie {
                Some(c) => parse_session_cookie(c),
                _ => (None, None)
            };

            Context { pool: pool.clone(), username: user, session_token: token }
        })
        .boxed()
}

fn parse_session_cookie(token_cookie: String) -> (Option<String>, Option<String>) {
    let mut splitted = token_cookie.split("##");
    match (splitted.nth(0), splitted.nth(0)) {
        (Some(user), Some(token)) => (Some(user.into()), Some(token.into())),
        _ => (None, None)
    }
}
