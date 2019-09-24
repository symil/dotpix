# dotpix

Module to take screenshots with Node.js.

## Install

```
npm install dotpix --save
```

## Usage

```js
const dotpix = require('dotpix');

// Returns an object with three fields: `width`, `height` and `data`
const image = dotpix.screenshot();

// Display the size of the image
console.log(image.width, image.height);

// Display the first pixel of the image (RGBA)
console.log(image.data.slice(0, 4));
```

## Additional information

dotpix is a wrapper around [Scrap](https://github.com/quadrupleslap/scrap) and uses [Neon](https://github.com/neon-bindings/neon) for the Rust->Node.js bindings.

- Rust must be installed in order to compile dotpix
- To use with electron, follow [this guide](https://neon-bindings.com/docs/electron-apps)
