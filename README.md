# Swatch
This is a simple Cloudflare worker to generate SVG images of color swatches. The only currently supported swatch format is a 2:1 gradient.

## Format
To get a swatch, you can hit `https://swatch.daedalus.workers.dev/<decimal encoded hex color>/gradient.svg`. For example, `#ff0049` would be encoded as `16711753`.

## Examples
![red](https://swatch.daedalus.workers.dev/16711753/gradient.svg)
![blue](https://swatch.daedalus.workers.dev/4362970/gradient.svg)