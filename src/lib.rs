use worker::*;
use urlencoding;
mod utils;

#[event(fetch)]
pub async fn main(req: worker::Request, env: Env, _ctx: worker::Context) -> worker::Result<worker::Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    Router::new()
        .get_async("/:query", |_req, ctx| async move {
            let query = ctx.param("query").unwrap();
            let cache = ctx.kv("OPENFOODS_CACHE").unwrap();
            let query_urldecoded = urlencoding::decode(query).unwrap().into_owned();

            if let Some(cached_result) = cache.get(&query).text().await.unwrap() {
                console_log!("found a cached result for query {}", query_urldecoded);
                return worker::Response::ok(cached_result);
            }

            console_log!("querying {}", query_urldecoded);

            if let Ok(response) = reqwest::Client::new()
                .get("https://world.openfoodfacts.org/cgi/search.pl")
                .query(&[
                    ("search_terms", query_urldecoded),
                    ("search_simple", String::from("1")),
                    ("action", String::from("process")),
                ])
                .send().await {
                    let html: &str = &response.text().await.unwrap()[..];
                    let products = utils::extract_products(html);
                    if products.is_ok() {
                        console_log!("found products, inserting into cache");
                        let products_json = products.unwrap();
                        cache.put(&query, products_json).unwrap().execute().await.unwrap();
                        return worker::Response::ok(products_json);
                    } else {
                        let mut out = products.unwrap_err().to_string();
                        out.push_str(&format!("\nopenfoodfacts.org server returned:\n{}", html)[..]);
                        return worker::Response::error(out, 500);
                    }
                }
            worker::Response::error("Internal Server Error", 500)
        }).run(req, env).await
}

