use aom_decode::{
  avif::{Avif, Image},
  Config,
};
use image::{codecs::jpeg::JpegEncoder, ColorType, DynamicImage, ImageBuffer, ImageFormat};
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

fn _load(bin: &[u8], ext: Option<&str>) -> anyhow::Result<DynamicImage> {
  let guessed;
  let format;

  #[allow(clippy::never_loop)]
  loop {
    if let Some(ext) = &ext {
      if ext == &"avif" {
        let avif = Avif::decode(bin, &Config { threads: 1 })?.convert()?;
        match avif {
          Image::RGB8(avif) => {
            let width = avif.width() as u32;
            let height = avif.height() as u32;
            let pxli = avif.pixels();
            let mut li = Vec::with_capacity(pxli.len() * 3);
            for px in pxli {
              li.push(px.r);
              li.push(px.g);
              li.push(px.b);
            }
            let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, li).unwrap();
            return Ok(img.into());
          }
          Image::RGB16(avif) => {
            let width = avif.width() as u32;
            let height = avif.height() as u32;
            let pxli = avif.pixels();
            let mut li = Vec::with_capacity(pxli.len() * 3);
            for px in pxli {
              li.push((px.r >> 8) as u8);
              li.push((px.g >> 8) as u8);
              li.push((px.b >> 8) as u8);
            }
            let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, li).unwrap();
            return Ok(img.into());
          }
          Image::RGBA8(avif) => {
            let width = avif.width() as u32;
            let height = avif.height() as u32;
            let pxli = avif.pixels();
            let mut li = Vec::with_capacity(pxli.len() * 3);
            // PNG pixel RGB = (a * r + (255 - a)) / 255, (a * g + (255 - a)) / 255, (a * b + (255 - a)) / 255
            for px in pxli {
              if px.a == 0 {
                li.push(255);
                li.push(255);
                li.push(255);
              } else if px.a == 255 {
                li.push(px.r);
                li.push(px.g);
                li.push(px.b);
              } else {
                let a = (px.a as f64) / 255.0;
                let bg = (1.0 - a) * 255.0;
                let r = px.r as f64;
                let g = px.g as f64;
                let b = px.b as f64;
                li.push((r * a + bg) as u8);
                li.push((g * a + bg) as u8);
                li.push((b * a + bg) as u8);
              }
            }
            let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, li).unwrap();
            return Ok(img.into());
          }
          Image::RGBA16(avif) => {
            let width = avif.width() as u32;
            let height = avif.height() as u32;
            let pxli = avif.pixels();
            let mut li = Vec::with_capacity(pxli.len() * 3);
            for px in pxli {
              if px.a == 0 {
                li.push(255);
                li.push(255);
                li.push(255);
              } else if px.a == 255 {
                li.push((px.r >> 8) as u8);
                li.push((px.g >> 8) as u8);
                li.push((px.b >> 8) as u8);
              } else {
                let a = (px.a as f64) / 255.0;
                let bg = (1.0 - a) * 255.0;
                let r = px.r as f64;
                let g = px.g as f64;
                let b = px.b as f64;
                li.push(((r * a + bg) as u16 >> 8) as u8);
                li.push(((g * a + bg) as u16 >> 8) as u8);
                li.push(((b * a + bg) as u16 >> 8) as u8);
              }
            }
            let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, li).unwrap();
            return Ok(img.into());
          }
          _ => {
            todo!();
          }
        }
        //   Image::Gray8(avif) => {
        //     let (out, width, height) = avif.into_contiguous_buf();
        //     let buf = ImageBuffer::from_raw(width, height, out);
        //     lodepng::encode_file(&out_path, &out, width, height, lodepng::ColorType::GREY, 8)
        //   }
        //   Image::Gray16(avif) => {
        //     let mut out = Vec::new();
        //     for px in avif.pixels() {
        //       out.push((px >> 8) as u8);
        //     }
        //     lodepng::encode_file(
        //       &out_path,
        //       &out,
        //       avif.width(),
        //       avif.height(),
        //       lodepng::ColorType::GREY,
        //       8,
        //     )
        //   }
      }
      if let Some(f) = ImageFormat::from_extension(ext) {
        guessed = false;
        format = f;
        break;
      }
    }
    guessed = true;
    format = image::guess_format(bin)?;
    break;
  }
  Ok(match image::load_from_memory_with_format(bin, format) {
    Ok(r) => r,
    Err(err) => {
      if guessed {
        Err(err)?;
      };
      image::load_from_memory_with_format(bin, image::guess_format(bin)?)?
    }
  })
}

fn _img_jpg(pkg: &Pkg) -> anyhow::Result<Buffer> {
  let img = _load(&pkg.bin, pkg.ext.as_deref())?;
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
