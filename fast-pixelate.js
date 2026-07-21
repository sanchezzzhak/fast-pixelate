const { join } = require('node:path');
let nativeBinding = null;

if (process.platform === 'linux' && process.arch === 'x64') {
    nativeBinding = require(join(__dirname, 'fast-pixelate.linux-x64-gnu.node'));
} else if (process.platform === 'win32' && process.arch === 'x64') {
    nativeBinding = require(join(__dirname, 'fast-pixelate.win32-x64-msvc.node'));
} else {
    throw new Error(`Platform ${process.platform} not supported`);
}

module.exports = {
    pixelate: nativeBinding.pixelate,
    pixelateAsync: nativeBinding.pixelateAsync,
};
