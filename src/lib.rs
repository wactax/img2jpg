// use image::EncodableLayout;
use image::{codecs::jpeg::JpegEncoder, ColorType, ImageFormat};
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
  let bin = &pkg.bin;
  let guessed;
  let format;

  #[allow(clippy::never_loop)]
  loop {
    if let Some(ext) = &pkg.ext {
      if let Some(f) = ImageFormat::from_mime_type(ext) {
        guessed = false;
        format = f;
        break;
      }
    }
    guessed = true;
    format = image::guess_format(bin)?;
    break;
  }

  let img = match image::load_from_memory_with_format(bin, format) {
    Ok(r) => r,
    Err(err) => {
      if guessed {
        Err(err)?;
      };
      image::load_from_memory_with_format(bin, image::guess_format(bin)?)?
    }
  };

  let mut bytes = Vec::new();
  let mut encoder = JpegEncoder::new_with_quality(&mut bytes, pkg.quality);
  encoder.encode(&img.as_bytes(), img.width(), img.height(), ColorType::Rgb8)?;
  Ok(bytes.into())
}

#[allow(dead_code)]
#[napi]
fn img_jpg(bin: Buffer, ext: Option<String>, quality: u8) -> AsyncTask<Pkg> {
  AsyncTask::new(Pkg { bin, quality, ext })
}
