#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var ROOT;

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

test('img → jpg', async(t) => {
  var r;
  r = (await imgJpg(readFileSync(join(ROOT, '1.avif')), 'avif', 80)); // https://docs.rs/jpegxl-rs/latest/jpegxl_rs/encode/struct.JpgEncoderBuilder.html#method.quality
  write(join(ROOT, '1.jpg'), r);
  t.true(r instanceof Buffer);
});

// t.pass()
