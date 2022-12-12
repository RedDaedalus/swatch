use worker::*;

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/:color/gradient.svg", |_, ctx| {
            if let Some(color) = ctx.param("color") {
                let color = color.parse::<i32>().unwrap();
                let svg = utils::compose_svg_gradient(color);

                let mut headers = Headers::new();
                headers.append("Content-Type", "image/svg+xml")?;
                headers.append("Cache-Control", "max-age=604800")?; // cache images for 1 week

                Ok(Response::ok(svg)?.with_headers(headers))
            } else {
                Response::error("Bad Request", 400)
            }
        })
        .get("/:color/solid.svg", |_, ctx| {
            if let Some(color) = ctx.param("color") {
                let color = color.parse::<i32>().unwrap();
                let svg = utils::compose_svg_solid(color);

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
