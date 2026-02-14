use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct TransformRequest {
    text: String,
}

#[derive(Serialize)]
struct TransformResponse {
    original: String,
    transformed: String,
}

fn cors_headers(headers: &mut Headers) {
    headers.set("Access-Control-Allow-Origin", "*").unwrap();
    headers.set("Access-Control-Allow-Methods", "POST, OPTIONS").unwrap();
    headers.set("Access-Control-Allow-Headers", "Content-Type").unwrap();
}

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.method() == Method::Options {
        let mut headers = Headers::new();
        cors_headers(&mut headers);
        return Ok(Response::empty()?.with_headers(headers).with_status(204));
    }

    if req.method() == Method::Post && req.path() == "/api/transform" {
        let mut req = req;
        let body: TransformRequest = req.json().await?;
        let transformed = yanify_transform::transform(&body.text);
        let response = TransformResponse {
            original: body.text,
            transformed,
        };
        let mut resp = Response::from_json(&response)?;
        cors_headers(resp.headers_mut());
        return Ok(resp);
    }

    Response::error("Not Found", 404)
}
