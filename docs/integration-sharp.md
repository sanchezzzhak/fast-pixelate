

Usage in sharp

```js
const {pixelateAsync} = require('fast-pixelate');
const sharp = require("sharp");

/**
 * 
 * @param rawBuffer  - sharp buffer image original file
 * @param metadata   - sharp metadata original file
 * @param detections - [{ box: [ 914, 619, 934, 645 ], score: 0.7481355667114258 }]
 * @param gridSize = 16
 * @param padding = 0.2
 * @return {Promise<string>}
 */
async function blurBuffer(
	rawBuffer,
	metadata,
	detections,
	gridSize = 16,
	padding = 0.2
) {
	const imgWidth = metadata.width || 0;
	const imgHeight = metadata.height || 0;
	const channels = metadata.channels || 3;

	const boxesOnly = detections.map(d => d.box);

	const processedRawBuffer = await pixelateAsync(
		rawBuffer,
		imgWidth,
		imgHeight,
		channels,
		boxesOnly,
		gridSize,
		padding
	);
	
	const finalSharp = await sharp(processedRawBuffer,
		{raw: {width: imgWidth, height: imgHeight, channels}})
		.webp({effort:6, nearLossless: true})
		.toBuffer()

	return finalSharp.toString('base64');
}

```