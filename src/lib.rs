use avif_img::load_image;
use image::{codecs::jpeg::JpegEncoder, ColorType};
use napi::{
  bindgen_prelude::{AsyncTask, Buffer},
  Env, Result, Task,
};
use napi_derive::napi;

struct Pkg {
  bin: Buffer,
  quality: u8,
  ext: Option<String>,
}

impl Task for Pkg {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(_img_jpg(self)?)
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

fn _img_jpg(pkg: &Pkg) -> anyhow::Result<Buffer> {
  let img = load_image(pkg.ext.as_deref(), &pkg.bin)?;
  let mut bytes = Vec::new();
  let mut encoder = JpegEncoder::new_with_quality(&mut bytes, pkg.quality);
  encoder.encode(img.as_bytes(), img.width(), img.height(), ColorType::Rgb8)?;
  Ok(bytes.into())
}

#[allow(dead_code)]
#[napi]
fn img_jpg(bin: Buffer, ext: Option<String>, quality: u8) -> AsyncTask<Pkg> {
  AsyncTask::new(Pkg { bin, quality, ext })
}
