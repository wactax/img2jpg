[‼️]: ✏️README.mdt

# @w5/img2jpg

## Install

```
pnpm i -g @w5/img2jpg
```

## Test

[coffee/main.coffee](./coffee/main.coffee) :

```coffee
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
    for img from ['rgba16','rgb8','rgb16','rgba8']
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
```

output :

```

  ✔ img → jpg (7s)
  ─

  1 test passed
```
