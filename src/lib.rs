use bmp_monochrome::Bmp;
use worker::*;

use image::{GenericImageView, Pixel};
use std::io::Cursor;
use worker::wasm_bindgen::UnwrapThrowExt;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

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
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get_async("/convert/chroma29", |req, _ctx| async move {
            let url = req.url()?;
            let invert = url
                .query_pairs()
                .find(|(key, _)| key == "inv")
                .map(|(_, val)| val)
                .unwrap_or(std::borrow::Cow::Borrowed("0"))
                == "1";
            let color = url
                .query_pairs()
                .find(|(key, _)| key == "color")
                .map(|(_, val)| val)
                .unwrap_or(std::borrow::Cow::Borrowed("0"))
                == "1";
            let rotate: i32 = url
                .query_pairs()
                .find(|(key, _)| key == "rotate")
                .map(|(_, val)| val)
                .unwrap_or(std::borrow::Cow::Borrowed("0"))
                .parse()
                .unwrap_or(0);
            let origin = url
                .query_pairs()
                .find(|(key, _)| key == "origin")
                .map(|(_, val)| val);
            if origin.is_none() {
                return Response::error("origin missing", 400);
            }

            let origin_url_str = origin.unwrap().to_string();
            let origin_url = Url::parse(&origin_url_str);
            if origin_url.is_err() {
                return Response::error("origin is not a URL", 400);
            }

            let origin_resp = Fetch::Url(origin_url.unwrap()).send().await;
            if let Err(err) = origin_resp {
                return Response::error(
                    format!(
                        "couldn't get origin({}): {}",
                        origin_url_str,
                        err.to_string(),
                    ),
                    400,
                );
            }
            let mut origin_resp = origin_resp.unwrap();
            if origin_resp.status_code() != 200 {
                return Response::error(
                    format!(
                        "couldn't get origin({}): status code {}\n{}",
                        origin_url_str,
                        origin_resp.status_code(),
                        origin_resp.text().await.unwrap_or_default(),
                    ),
                    400,
                );
            }
            let bytes = origin_resp.bytes().await;
            if let Err(err) = bytes {
                return Response::error(
                    format!(
                        "couldn't read origin({}): {}",
                        origin_url_str,
                        err.to_string(),
                    ),
                    400,
                );
            }
            let img = image::load_from_memory(bytes.unwrap().as_mut_slice());

            if let Err(err) = img {
                return Response::error(format!("couldn't decode image: {}", err.to_string()), 400);
            }
            let mut img = img.unwrap_throw();
            if rotate > 0 {
                console_log!("rotating by {}", rotate);
                match rotate {
                    90 => img = img.rotate90(),
                    180 => img = img.rotate180(),
                    270 => img = img.rotate270(),
                    _ => console_log!("invalid degrees {}", rotate),
                }
            }
            let (w, h) = img.dimensions();
            console_log!("image: w={} h={}", w, h);

            let mut bmp_data = Vec::with_capacity(h.try_into().unwrap());
            for y in 0..h {
                let mut pixels = Vec::with_capacity(w.try_into().unwrap());
                for x in 0..w {
                    let pixel = img.get_pixel(x, y);
                    //Saturation = [(MaxColor - MinColor) / (MaxColor + MinColor)]
                    let max_col = *pixel.to_rgb().channels().iter().max().unwrap() as f32;
                    let min_col = *pixel.to_rgb().channels().iter().min().unwrap() as f32;
                    let sat = (max_col - min_col) / 255_f32.min(max_col + min_col);
                    let int: f32 = pixel
                        .to_rgb()
                        .channels()
                        .iter()
                        .fold(0_f32, |s, c| s + (*c as f32) / 255.0)
                        / 3_f32;
                    let pixel_active = if color { sat > 0.75 } else { int < 0.25 };
                    pixels.push(pixel_active ^ invert);
                }
                bmp_data.push(pixels);
            }
            let bmp = Bmp::new(bmp_data).unwrap();
            let mut bmp_buf = Cursor::new(Vec::new());
            bmp.write(&mut bmp_buf).unwrap();
            let mut headers = Headers::new();
            headers.set("Content-Type", "image/bmp").unwrap();
            Ok(Response::from_bytes(bmp_buf.into_inner())
                .unwrap()
                .with_headers(headers))
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
