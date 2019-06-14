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
const firstPixel = image.data.slice(0, 4);

console.log(image.width);
console.log(image.height);
console.log(firstPixel);
```

## Requirements

- Rust must be installed
- To use with electron, follow [this guide](https://neon-bindings.com/docs/electron-apps)