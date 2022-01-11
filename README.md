# Creating 1bit BMPs for eInk displays from PNGs

A serverless function hosted on Cloudflare workers to create images that are compatible with eInk displays as such as supported by https://github.com/atc1441/E-Paper_Pricetags/tree/main/Custom_PriceTag_AccesPoint .

Hosted at https://eink-bmp-converter.h32.workers.dev

## Parameters
* **origin**: the source URL to pull a PNG from
* **inv**: 0/1 to indicate if the image should be inverted

## Example

https://eink-bmp-converter.h32.workers.dev/convert/chroma29?origin=https://s20.directupload.net/images/220111/wtuiao47.png&inv=1
creates this:

![sample image](https://eink-bmp-converter.h32.workers.dev/convert/chroma29?origin=https://s20.directupload.net/images/220111/wtuiao47.png&inv=1)
