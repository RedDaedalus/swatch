# Swatch
This is a simple Cloudflare worker to generate SVG images of color swatches. The only currently supported swatch format is a 2:1 gradient.

## Format
To get a swatch, you can hit `https://swatch.daedalus.workers.dev/<url encoded CSS color>/gradient.svg`. Note that this is vulnerable to XSS in the sense that the color is arbitrarily inserted into the SVG, so make sure to either embed as an image rather than inserting into your HTML or to sanitize inputs if it is a problem.

## Examples
![red](https://swatch.daedalus.workers.dev/%23ff0049/gradient.svg)
![blue](https://swatch.daedalus.workers.dev/%234292da/gradient.svg)