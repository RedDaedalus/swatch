use worker::*;
use urlencoding::decode;

mod utils;

fn compose_svg(color: &String) -> String {
    return format!("
        <svg width=\"512\" height=\"256\" viewbox=\"0 0 512 256\" xmlns=\"http://www.w3.org/2000/svg\">
            <defs>
                <linearGradient id=\"g\" x2=\"100%\" y2=\"100%\">
                    <stop offset=\"0%\" stop-color=\"black\" stop-opacity=\"0\" />
                    <stop offset=\"100%\" stop-color=\"black\" stop-opacity=\"0.4\" />
                </linearGradient>
            </defs>

            <rect rx=\"4\" ry=\"4\" width=\"512\" height=\"256\" fill=\"{}\" />
            <rect rx=\"4\" ry=\"4\" width=\"512\" height=\"256\" fill=\"url(#g)\" />
        </svg>
    ", color);
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/:color/gradient.svg", |_, ctx| {
            if let Some(color) = ctx.param("color") {
                let color = decode(color).expect("UTF-8").into_owned();
                let svg = compose_svg(&color);

                let mut headers = Headers::new();
                headers.append("Content-Type", "image/svg+xml")?;
                // headers.append("Content-Disposition", "inline")?;

                Ok(Response::ok(svg)?.with_headers(headers))
            } else {
                Response::error("Bad Request", 400)
            }
        })
        .run(req, env)
        .await
    // router
    //     .get("/", |_, _| Response::ok("Hello from Workers!"))
    //     .post_async("/form/:field", |mut req, ctx| async move {
    //         if let Some(name) = ctx.param("field") {
    //             let form = req.form_data().await?;
    //             match form.get(name) {
    //                 Some(FormEntry::Field(value)) => {
    //                     return Response::from_json(&json!({ name: value }))
    //                 }
    //                 Some(FormEntry::File(_)) => {
    //                     return Response::error("`field` param in form shouldn't be a File", 422);
    //                 }
    //                 None => return Response::error("Bad Request", 400),
    //             }
    //         }

    //         Response::error("Bad Request", 400)
    //     })
    //     .get("/worker-version", |_, ctx| {
    //         let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
    //         Response::ok(version)
    //     })
}
