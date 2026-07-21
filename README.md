# fast-pixelate 🚀

![img.png](docs%2Fimg.png)

- High-performance image pixelation and censoring library for Node.js written in Rust. Blazing fast, asynchronous-friendly, and lightweight.

- Optimized specifically for **privacy compliance, automated data redaction, and real-time image masking** pipelines (e.g., blurring or pixelating faces, credit cards, license plates, or sensitive documents). It performs bounding-box calculation, percentage-based padding adjustments, and pixel-grid rendering directly on raw buffers with native CPU acceleration.

## ✨ Features

- **Rust-powered Speed:** Up to 10-50x faster than pure JavaScript canvas-based or loop implementations.
- **True Multi-threading:** `*Async` methods execute on the libuv thread pool without blocking the main Node.js event loop.
- **Zero-Copy In-Place Modification:** Modifies the native Node.js `Buffer` directly without extra allocations or data copying.
- **Dual Support:** Ships with both **CommonJS** and **ESM** exports out of the box.
- **Cross-Platform:** Pre-compiled binaries for Linux (gnu) and Windows (msvc) x86_64.

## 📦 Installation

```bash
npm install fast-pixelate
```

*Linux: By default, pre-compiled binaries will be used. If your system requires compiling from source:*
```bash
FAST_PIXELATE_COMPILE=true npm install fast-pixelate
```
*Windows*
```bash
set FAST_PIXELATE_COMPILE=true && npm install fast-pixelate
```

## 🚀 Quick Start

Here is how to asynchronously pixelate faces on a raw image buffer using object detection coordinates:

```typescript
import { pixelateAsync } from 'fast-pixelate';
import * as fs from 'fs';

const imgWidth = 1920;
const imgHeight = 1080;
const channels = 4;
const rawImgBuffer = await getBoxBuffer();

// Array of bounding boxes detected on the image: [[x1, y1, x2, y2], ...]
const boxes = [, // Face 1 coordinates
  [750, 400, 910, 580]  // Face 2 coordinates
];

const gridSize = 16; // Size of the pixel blocks (must be > 0)
const padding = 0.1; // Add 10% extra padding around bounding boxes to ensure full coverage

async function run() {
  // Executes in a background worker thread, no event loop blocking!
  // Modifies rawImgBuffer in-place
  await pixelateAsync(rawImgBuffer, imgWidth, imgHeight, channels, boxes, gridSize, padding);
  console.log('Image censored successfully. Buffer modified in-place.');
}

run();
```

## 📖 API Reference

The package provides both **synchronous** (blocking) and **asynchronous** (non-blocking, Promise-based) methods for processing packed pixel buffers.

### Functions

```typescript
/**
 * Synchronously pixelates specified regions inside an image buffer in-place.
 * Blocks the Event Loop, ideal for micro-tasks or CLI tools.
 */
export function pixelate(
    buffer: Buffer,
    imgWidth: number,
    imgHeight: number,
    channels: number,
    boxes: Array<Array<number>>,
    gridSize: number,
    padding: number
): Buffer;

/**
 * Asynchronously pixelates specified regions inside an image buffer in-place.
 * Offloads processing to the libuv thread pool, keeping your server fully responsive.
 */
export function pixelateAsync(
    buffer: Buffer,
    imgWidth: number,
    imgHeight: number,
    channels: number,
    boxes: Array<Array<number>>,
    gridSize: number,
    padding: number
): Promise<Buffer>;
```

#### Arguments:
- `buffer`: Native Node.js `Buffer` containing raw image pixels (HWC layout). Modified in-place.
- `imgWidth` / `imgHeight`: Dimensions of the image in pixels.
- `channels`: Number of color channels (`3` for RGB, `4` for RGBA).
- `boxes`: Two-dimensional array representing bounding boxes to hide `[[x1, y1, x2, y2], ...]`.
- `gridSize`: Resolution of the pixelation block. Higher values create larger pixel blocks (must be `> 0`).
- `padding`: Multiplier for expanding the bounding box size (e.g., `0.1` expands the area by 10% in all directions).

## 🛠️ Supported Targets

- `x86_64-unknown-linux-gnu` (Linux x64)
- `x86_64-pc-windows-msvc` (Windows x64)

## Some usage examples

See view file [integration-sharp.md](docs%2Fintegration-sharp.md) for automated masking pipelines using popular image processing libraries.

## 📄 License

MIT
