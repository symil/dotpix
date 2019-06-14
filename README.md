# dotpix

Module to take screenshots with Node.js.

## Install

```
npm install dotpix
```

## Usage

```js
const dotpix = require('dotpix');

// Returns an object with three fields: `width`, `height` and `data`
const image = dotpix.screenshot();

// Display the size of the image
console.log(image.width);
console.log(image.height);

// Display the first pixel of the image (RGBA)
console.log(image.data.slice(0, 4));
```

## Additional information

dotpix is written in Rust and uses [Neon](https://github.com/neon-bindings/neon) for the Node.js bindings.

- Rust must be installed in order to compile dotpix
- To use with electron, follow [this guide](https://neon-bindings.com/docs/electron-apps)