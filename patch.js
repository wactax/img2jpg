export default (img, ext = undefined, quality = 80) =>
	nativeBinding.imgJpg(img, ext, quality);
