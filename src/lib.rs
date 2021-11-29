use wasm_bindgen::prelude::*;

fn get_opts() -> usvg::Options {
  let mut fontdb = usvg::fontdb::Database::new();
  fontdb.load_font_data(include_bytes!("../fonts/Bitter-Bold.ttf").to_vec());
  fontdb.load_font_data(include_bytes!("../fonts/Bitter-Regular.ttf").to_vec());
  fontdb.load_font_data(include_bytes!("../fonts/Inter-Bold.ttf").to_vec());
  fontdb.load_font_data(include_bytes!("../fonts/Inter-Regular.ttf").to_vec());
  fontdb.load_font_data(
    include_bytes!("../fonts/JetBrainsMono-VariableFont_wght.ttf").to_vec(),
  );
  fontdb.set_serif_family("Bitter");
  fontdb.set_sans_serif_family("Inter");
  fontdb.set_monospace_family("JetBrains Mono");
  usvg::Options {
    fontdb,
    font_family: "Bitter".to_string(),
    ..Default::default()
  }
}

#[wasm_bindgen]
pub fn render(svg: String) -> Result<js_sys::Uint8Array, JsValue> {
  let opt = get_opts();

  let data = svg.as_bytes();
  let rtree = usvg::Tree::from_data(data, &opt.to_ref())
    .map_err(|err| JsValue::from(js_sys::Error::new(&err.to_string())))?;
  let pixmap_size = rtree.svg_node().size.to_screen_size();
  let mut pixmap =
    tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
      .unwrap_throw();
  resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap_throw();
  Ok(pixmap.encode_png().unwrap().as_slice().into())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_font_faces() {
    let opt = get_opts();
    for f in opt.fontdb.faces() {
      println!("{}", f.family);
    }
  }
}
