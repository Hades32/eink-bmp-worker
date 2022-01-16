# Creating 1bit BMPs for eInk displays from PNGs

A serverless function hosted on Cloudflare workers to create images that are compatible with eInk displays as such as supported by https://github.com/atc1441/E-Paper_Pricetags/tree/main/Custom_PriceTag_AccesPoint or my simplified fork https://github.com/Hades32/web-tags .

Hosted at https://eink-bmp-converter.h32.workers.dev

## Parameters
* **origin**: the source URL to pull a PNG from
* **color**: 0(default): only pixels with a intensity <25% become black. 1: only colorful pixels (saturation >75%) become black
* **inv**: 0/1 inverts the resulting image
* **rotate**: rotes the image by 90/180/270 degrees

## Example

| input | extract black, rotate | + invert | extract color, rotate | + invert |
|---|---|---|---|---|
| [![quote of the day](https://info-draw-worker.5gp.de/image)](https://info-draw-worker.5gp.de/image) <br> from https://github.com/Hades32/info-pic-worker | [![alt](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?color=0&inv=0&rotate=90&origin=https://info-draw-worker.5gp.de/image)](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?color=0&inv=0&rotate=90&origin=https://info-draw-worker.5gp.de/image)  | [![alt](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?color=0&inv=1&rotate=90&origin=https://info-draw-worker.5gp.de/image)](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?color=0&inv=1&rotate=90&origin=https://info-draw-worker.5gp.de/image)  | [![alt](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?color=1&inv=0&rotate=90&origin=https://info-draw-worker.5gp.de/image)](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?color=1&inv=0&rotate=90&origin=https://info-draw-worker.5gp.de/image) | [![alt](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?color=1&inv=1&rotate=90&origin=https://info-draw-worker.5gp.de/image)](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?color=1&inv=1&rotate=90&origin=https://info-draw-worker.5gp.de/image) |
