export default (img, ext = undefined, quality = 1) =>
	nativeBinding.imgJxl(img, ext, quality);
