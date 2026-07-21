import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

let nativeBinding = null;

if (process.platform === 'linux' && process.arch === 'x64') {
	nativeBinding = require(join(__dirname, 'fast-pixelate.linux-x64-gnu.node'));
} else if (process.platform === 'win32' && process.arch === 'x64') {
	nativeBinding = require(join(__dirname, 'fast-pixelate.win32-x64-msvc.node'));
} else {
	throw new Error(`Platform ${process.platform} not supported`);
}

export const pixelate = nativeBinding.pixelate;
export const pixelateAsync = nativeBinding.pixelateAsync;

export default {
	pixelate,
	pixelateAsync,
};
