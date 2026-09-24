use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

const MAX_EDGE: f64 = 1536.0;
const JPEG_QUALITY: f64 = 0.85;

/// Long edge ≤ 1536. Transparent images stay PNG; everything else is JPEG 0.85.
/// A failed compression returns the original data URL.
pub async fn compress_image(data_url: String) -> String {
    match compress_inner(&data_url).await {
        Ok(compressed) if compressed.len() < data_url.len() => compressed,
        _ => data_url,
    }
}

async fn compress_inner(data_url: &str) -> Result<String, JsValue> {
    let img = load_image(data_url).await?;
    let scale = (MAX_EDGE / img.width().max(img.height()) as f64).min(1.0);
    let width = ((img.width() as f64) * scale).round().max(1.0) as u32;
    let height = ((img.height() as f64) * scale).round().max(1.0) as u32;

    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    canvas.set_width(width);
    canvas.set_height(height);
    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .ok_or("no 2d")?
        .dyn_into()?;
    ctx.draw_image_with_html_image_element_and_dw_and_dh(
        &img,
        0.0,
        0.0,
        width as f64,
        height as f64,
    )?;

    let transparent = has_transparency(&ctx, width, height)?;
    if transparent {
        canvas.to_data_url_with_type("image/png")
    } else {
        canvas.to_data_url_with_type_and_encoder_options(
            "image/jpeg",
            &JsValue::from_f64(JPEG_QUALITY),
        )
    }
}

fn has_transparency(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
) -> Result<bool, JsValue> {
    let data = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let bytes = data.data();
    let mut i = 3;
    while i < bytes.len() {
        if bytes[i] < 255 {
            return Ok(true);
        }
        i += 64;
    }
    Ok(false)
}

async fn load_image(src: &str) -> Result<HtmlImageElement, JsValue> {
    let img = HtmlImageElement::new()?;
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let onload = Closure::once(move || {
            let _ = resolve.call0(&JsValue::NULL);
        });
        let onerror = Closure::once(move || {
            let _ = reject.call0(&JsValue::NULL);
        });
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onload.forget();
        onerror.forget();
    });
    img.set_src(src);
    JsFuture::from(promise).await?;
    Ok(img)
}
