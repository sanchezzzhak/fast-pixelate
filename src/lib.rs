use napi_derive::napi;
use napi::{bindgen_prelude::*, Task, Env, Result};

fn internal_pixelate(
    buffer: &mut [u8],
    img_width: i32,
    img_height: i32,
    channels: i32,
    boxes: &[Vec<i32>],
    grid_size: i32,
    padding: f64,
) {
    let channels = channels as usize;
    let img_width_usize = img_width as usize;

    for bbox in boxes {
        if bbox.len() < 4 { continue; }
        let x1 = bbox[0];
        let y1 = bbox[1];
        let x2 = bbox[2];
        let y2 = bbox[3];

        let w = x2 - x1;
        let h = y2 - y1;

        // Calculating floating point bounds and padding
        let left = (x1 as f64 - w as f64 * padding).round() as i32;
        let left = left.max(0);

        let top = (y1 as f64 - h as f64 * padding).round() as i32;
        let top = top.max(0);

        let width = (w as f64 * (1.0 + padding * 2.0)).round() as i32;
        let width = width.min(img_width - left);

        let height = (h as f64 * (1.0 + padding * 2.0)).round() as i32;
        let height = height.min(img_height - top);

        if width <= 0 || height <= 0 { continue; }

        let right = left + width;
        let bottom = top + height;

        // Iterating over grid blocks gridSize
        let mut by = top;
        while by < bottom {
            let mut bx = left;
            while bx < right {
                // Calculate the central pixel of the block
                let cx = (bx + (grid_size >> 1)).min(right - 1) as usize;
                let cy = (by + (grid_size >> 1)).min(bottom - 1) as usize;

                let pixel_idx = (cy * img_width_usize + cx) * channels;
                if pixel_idx + 2 >= buffer.len() { break; }

                // Extracting the color of the central pixel
                let r = buffer[pixel_idx];
                let g = buffer[pixel_idx + 1];
                let b = buffer[pixel_idx + 2];
                let a = if channels == 4 && pixel_idx + 3 < buffer.len() {
                    Some(buffer[pixel_idx + 3])
                } else {
                    None
                };

                let block_max_y = (by + grid_size).min(bottom);
                let block_max_x = (bx + grid_size).min(right);

                // Filling the block with color
                for y in by..block_max_y {
                    let row_offset = (y as usize) * img_width_usize * channels;
                    for x in bx..block_max_x {
                        let target_idx = row_offset + (x as usize) * channels;
                        if target_idx + 2 < buffer.len() {
                            buffer[target_idx] = r;
                            buffer[target_idx + 1] = g;
                            buffer[target_idx + 2] = b;
                            if let (Some(alpha), true) = (a, channels == 4) {
                                if target_idx + 3 < buffer.len() {
                                    buffer[target_idx + 3] = alpha;
                                }
                            }
                        }
                    }
                }

                bx += grid_size;
            }
            by += grid_size;
        }
    }
}

#[napi]
pub fn pixelate(
    mut buffer: Buffer,
    img_width: i32,
    img_height: i32,
    channels: i32,
    boxes: Vec<Vec<i32>>,
    grid_size: i32,
    padding: f64,
) -> Buffer {
    let buf_slice = buffer.as_mut();
    internal_pixelate(buf_slice, img_width, img_height, channels, &boxes, grid_size, padding);
    buffer
}

pub struct PixelateTask {
    buffer: Buffer,
    img_width: i32,
    img_height: i32,
    channels: i32,
    boxes: Vec<Vec<i32>>,
    grid_size: i32,
    padding: f64,
}

impl Task for PixelateTask {
    type Output = Buffer;
    type JsValue = Buffer;

    // Этот метод выполняется в фоновом системном потоке (Worker Thread)
    fn compute(&mut self) -> Result<Self::Output> {
        let buf_slice = self.buffer.as_mut();
        internal_pixelate(
            buf_slice,
            self.img_width,
            self.img_height,
            self.channels,
            &self.boxes,
            self.grid_size,
            self.padding
        );

        Ok(std::mem::take(&mut self.buffer))
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

#[napi]
pub fn pixelate_async(
    buffer: Buffer,
    img_width: i32,
    img_height: i32,
    channels: i32,
    boxes: Vec<Vec<i32>>,
    grid_size: i32,
    padding: f64,
) -> AsyncTask<PixelateTask> {
    let task = PixelateTask {
        buffer,
        img_width,
        img_height,
        channels,
        boxes,
        grid_size,
        padding,
    };
    AsyncTask::new(task)
}
