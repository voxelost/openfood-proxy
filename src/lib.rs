use worker::*;
use urlencoding;
mod utils;

#[event(fetch)]
pub async fn main(req: worker::Request, env: Env, _ctx: worker::Context) -> worker::Result<worker::Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    Router::new()
        .get_async("/:query", |_req, ctx| async move {
            let query = urlencoding::decode(ctx.param("query").unwrap()).unwrap().into_owned();
            console_log!("querying {}", query);

            if let Ok(response) = reqwest::Client::new()
                .get("https://world.openfoodfacts.org/cgi/search.pl")
                .query(&[
                    ("search_terms", query),
                    ("search_simple", String::from("1")),
                    ("action", String::from("process")),
                ])
                .send().await {
                    let html: &str = &response.text().await.unwrap()[..];
                    let products = utils::extract_products(html);
                    if products.is_ok() {
                        return worker::Response::ok(products.unwrap());
                    } else {
                        let mut out = products.unwrap_err().to_string();
                        out.push_str(&format!("\nopenfoodfacts.org server returned:\n{}", html)[..]);
                        return worker::Response::error(out, 500);
                    }
                }
            worker::Response::error("Internal Server Error", 500)
        }).run(req, env).await
}

