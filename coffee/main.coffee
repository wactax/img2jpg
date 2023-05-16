#!/usr/bin/env coffee

> ../index.js:imgJpg
  ava:test
  path > join dirname
  @w5/uridir
  @w5/write
  # @w5/read
  fs > readFileSync

ROOT = dirname uridir import.meta

test(
  'img → jpg'
  (t) =>
    r = await imgJpg(
      readFileSync join ROOT, '1.avif'
      'avif'
      0.8 # https://docs.rs/jpegxl-rs/latest/jpegxl_rs/encode/struct.JpgEncoderBuilder.html#method.quality
    )
    write(
      join(ROOT, '1.jpg')
      r
    )
    t.true(r instanceof Buffer)
    # t.pass()
    return
)
