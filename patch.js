export default (img, ext = undefined, quality = 1) =>
	nativeBinding.imgJpg(img, ext, quality);
