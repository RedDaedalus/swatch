use worker::*;
use urlencoding::decode;

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/:color/gradient.svg", |_, ctx| {
            if let Some(color) = ctx.param("color") {
                let color = decode(color).expect("UTF-8").into_owned();
                let svg = utils::compose_svg(&color);

                let mut headers = Headers::new();
                headers.append("Content-Type", "image/svg+xml")?;
                headers.append("Cache-Control", "max-age=604800")?; // cache images for 1 week

                Ok(Response::ok(svg)?.with_headers(headers))
            } else {
                Response::error("Bad Request", 400)
            }
        })
        .run(req, env)
        .await
}
