#!/usr/bin/env coffee

> ../index.js:imgJpg
  ava:test
  path > join dirname
  @w5/uridir
  @w5/write
  # @w5/read
  fs > readFileSync

ROOT = dirname uridir import.meta
IMG = join ROOT, 'img'

test(
  'img → jpg'
  (t) =>
    for img from ['transparency','1']
      r = await imgJpg(
        readFileSync join IMG, img+'.avif'
        'avif'
        80 # https://docs.rs/jpegxl-rs/latest/jpegxl_rs/encode/struct.JpgEncoderBuilder.html#method.quality
      )
      write(
        join(IMG, img+'.jpg')
        r
      )
      t.true(r instanceof Buffer)
    # t.pass()
    return
)
