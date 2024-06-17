#![allow(non_snake_case)] //i like non-snake-case!!!

use actix_web::{get, services, web, HttpRequest, HttpResponse, Scope};
use chrono::prelude::*;
use resvg::render;
use std::{error::Error, ops::Deref};
use tiny_skia::{Pixmap, Transform};
use usvg::{Options, Tree};

#[get("/paper/triple/{paperId}")]
async fn deperate_image(_req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(include_str!("../assets/bg.svg"))
        .unwrap();
    let options = Options::default();
    let ntime = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    let globals = liquid::object!({
        "text": "本功能已更新，请升级至最新版本。",
        "time": ntime.format("%Y-%m-%d %H:%M:%S").to_string()
    });
    let mut fontdb = fontdb::Database::new();
    fontdb.load_fonts_dir(unsafe {
        let data = crate::FONTPATH.take().unwrap();
        crate::FONTPATH = Some(data.clone());
        data
    });
    let mut pixmap = Pixmap::new(1200, 720)
        .ok_or("Pixmap allocation error")
        .unwrap();
    let svg = template.render(&globals)?;
    let tree = Tree::from_str(&svg, &options, &fontdb)?;
    render(&tree, Transform::default(), &mut (pixmap.as_mut()));
    let encoded_buffer =
        webp::Encoder::new(pixmap.data_mut(), webp::PixelLayout::Rgba, 1200, 720).encode_lossless();
    let result = encoded_buffer.deref().to_owned();
    Ok(HttpResponse::Ok().content_type("image/webp").body(result))
}

pub fn service() -> Scope {
    let services = services![deperate_image];
    web::scope("/api/img").service(services)
}
