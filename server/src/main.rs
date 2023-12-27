use server::{AlphaSpaceNemeric, Order, Row};
use std::{collections::HashMap, sync::OnceLock};

use axum::{
    body::StreamBody,
    extract::{Path, Query},
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::{Local, TimeZone};
use rand::Rng;
use tokio_util::io::ReaderStream;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
};

static TABLE: OnceLock<Vec<Row>> = OnceLock::new();

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("server start");

    let rows = (0..50)
        .map(|i| {
            let title = rand::thread_rng()
                .sample_iter(rand::distributions::Alphanumeric)
                .take(20)
                .map(char::from)
                .collect();

            let content = rand::thread_rng()
                .sample_iter(AlphaSpaceNemeric)
                .take(500)
                .map(char::from)
                .collect();

            let now = chrono::Local::now();

            let midnight = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
            let reg_plus = chrono::Duration::minutes(i);
            let reg_date = Local
                .from_local_datetime(&midnight)
                .unwrap()
                .checked_add_signed(reg_plus)
                .unwrap();

            let ellip = now
                .time()
                .signed_duration_since(midnight.time())
                .num_seconds();

            let mod_rand = rand::thread_rng().gen_range(-ellip..=ellip);
            let mod_date = if mod_rand > 0 {
                let dur_rand = chrono::Duration::seconds(mod_rand);
                Some(reg_date.checked_add_signed(dur_rand).unwrap())
            } else {
                None
            };

            Row {
                id: i as i32,
                title,
                content,
                reg_date,
                mod_date,
            }
        })
        .collect();

    TABLE.set(rows).unwrap_or_default();

    let app = Router::new()
        .route("/", get(root))
        .route("/houu", get(route_houu))
        .route("/json", get(route_json))
        .route("/error", get(route_error))
        .route("/download/:filename", get(route_download))
        .route("/:param", get(wildcard))
        .layer(
            ServiceBuilder::new()
                .layer(SetResponseHeaderLayer::overriding(
                    header::CONTENT_SECURITY_POLICY,
                    HeaderValue::from_static("default-src 'none'; img-src 'self'; style-src 'self' 'unsafe-inline' https://unpkg.com; font-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval' https://unpkg.com"),
                ))
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        .allow_origin(Any),
                ),
        );

    axum::Server::bind(&"0.0.0.0:7878".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> String {
    tracing::info!("[root] accessed");

    String::from("77")
}

async fn route_houu() -> String {
    tracing::info!("[houu] accessed");

    String::from("houu!")
}

async fn route_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "no!")
}

async fn route_json(Query(query): Query<HashMap<String, String>>) -> axum::Json<serde_json::Value> {
    tracing::info!("[json] accessed");

    let rows = TABLE.get().unwrap();

    let mut filter: Vec<&Row> = rows.iter().collect();

    let order = if let Some(order) = query.get("order") {
        order.to_owned().into()
    } else {
        Order::default()
    };

    filter.sort_by(|&a, &b| {
        let cmp = match query.get("sort").map(|sort| sort.as_str()) {
            Some("title") => a.title.cmp(&b.title),
            Some("content") => a.content.cmp(&b.content),
            Some("reg_date") => a.reg_date.cmp(&b.reg_date),
            Some("mod_date") => a.mod_date.cmp(&b.mod_date),
            _ => a.id.cmp(&b.id),
        };

        if order == Order::ASC {
            cmp
        } else {
            cmp.reverse()
        }
    });

    if let Some(limit) = query.get("limit") {
        if let Ok(limit) = limit.parse::<usize>() {
            if let Some(offset) = query.get("offset") {
                if let Ok(offset) = offset.parse::<usize>() {
                    filter = filter.into_iter().skip(offset).collect();
                }
            }
            filter = filter.into_iter().take(limit).collect();
        }
    }

    axum::Json(serde_json::to_value(filter).unwrap())
}

async fn route_download(Path(filename): Path<String>) -> impl IntoResponse {
    tracing::info!("[download] accessed");

    let path = match std::env::current_dir() {
        Ok(curr) => {
            if filename.contains('.') {
                curr.join(&filename)
            } else {
                curr.join(format!("{}.txt", filename))
            }
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get current dir: {}", err),
            ))
        }
    };

    let file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                tracing::warn!("Not found: {}", path.display());
                let content: String = if filename == "atoz.txt" {
                    ('a'..='z').cycle().take(256 * 1024 * 1024).collect()
                } else {
                    filename.chars().cycle().take(1024 * 1024).collect()
                };

                if let Err(e) = tokio::fs::write(&path, content).await {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to create file: {}", e),
                    ));
                };
                match tokio::fs::File::open(&path).await {
                    Ok(f) => f,
                    Err(e) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to open file: {}", e),
                        ))
                    }
                }
            } else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to open file: {}", err),
                ));
            }
        }
    };

    let mut headers = HeaderMap::new();

    headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );

    let disposition = path
        .file_name()
        .and_then(|filename| filename.to_str())
        .and_then(|filename| {
            let disposition = format!("attachment; filename=\"{}\"", filename);
            HeaderValue::from_str(disposition.as_str()).ok()
        });

    match disposition {
        Some(disp) => headers.append(header::CONTENT_DISPOSITION, disp),
        None => headers.append(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_static("attachment; filename=\"unknown\""),
        ),
    };

    if let Ok(len) = file.metadata().await.map(|metadata| metadata.len()) {
        headers.append(header::CONTENT_LENGTH, HeaderValue::from(len));
    }

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Ok((headers, body))
}

async fn wildcard(Path(param): Path<String>) -> String {
    tracing::info!("/{} accessed", param);

    format!("\"{}\" 같은 건 없어요~", param)
}
