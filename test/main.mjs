#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var IMG, ROOT;

import imgJpg from '../index.js';

import test from 'ava';

import {
  join,
  dirname
} from 'path';

import uridir from '@w5/uridir';

import write from '@w5/write';

import {
  // @w5/read
  readFileSync
} from 'fs';

ROOT = dirname(uridir(import.meta));

IMG = join(ROOT, 'img');

test('img → jpg', async(t) => {
  var img, r, ref;
  ref = ['transparency', '1'];
  for (img of ref) {
    r = (await imgJpg(readFileSync(join(IMG, img + '.avif')), 'avif', 80)); // https://docs.rs/jpegxl-rs/latest/jpegxl_rs/encode/struct.JpgEncoderBuilder.html#method.quality
    write(join(IMG, img + '.jpg'), r);
    t.true(r instanceof Buffer);
  }
});

// t.pass()
