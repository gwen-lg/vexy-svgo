const { optimize } = require('vexy-svgo-wasm');

const svg = '<svg><g><rect width="100" height="100"/></g></svg>';

const result = optimize(svg);

console.log(result.data);
