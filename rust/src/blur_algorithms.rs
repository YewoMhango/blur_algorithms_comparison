/// # Basic Box Blur
/// A naive, first implementation of the box blur algorithm,
/// which just sums all the numbers within the given radius for each
/// pixel and divides by their count to find the average. It quickly
/// gets very slow as radius increases since the complexity of
/// determining the sum is O(r²) with respect to radius r
pub fn box_blur(band: &Vec<u8>, width: i32, height: i32, radius: i32) -> Vec<u8> {
    let mut new_band = band.clone();

    let mut i: i32 = 0;

    for h in 0..height {
        for w in 0..width {
            let mut cell_count: i32 = 0;
            let mut sum: i32 = 0;

            for m in (-radius)..radius {
                for n in (-radius)..radius {
                    if m + h < height && m + h > -1 && n + w < width && n + w > -1 {
                        cell_count += 1;
                        sum += band[(i + (width * m + n)) as usize] as i32;
                    }
                }
            }

            new_band.push((sum / cell_count) as u8);
            i += 1;
        }
    }

    new_band
}

/// # Optimized Box Blur
/// Compared to the basic box blur, this version finds
/// the average for pixels whithin the radius of each
/// pixel row-wise only and then column-wise.
///
/// Additionally, we do not calculate the sum all over
/// again for each pixel. Instead, we take the sum from
/// the previous computation and add the newest pixel
/// to enter within the radius of the current pixel,
/// while subtracting the value of the pixel that has
/// exited it.
///
/// This makes sum computation to be an
/// (almost) O(1) operation since it only needs to add
/// and subtract 2 numbers from sum regardless of radius
pub fn box_blur_optimized(band: &Vec<u8>, width: i32, height: i32, radius: i32) -> Vec<u8> {
    let mut first_new_band: Vec<u8> = vec![];
    let mut second_new_band: Vec<u8> = vec![];

    first_new_band.resize((width * height) as usize, 0 as u8);
    second_new_band.resize((width * height) as usize, 0 as u8);

    for h in 0..height {
        let mut sum: i32 = 0;
        let mut cell_count: i32 = radius + 1;

        for j in 0..(radius + 1) {
            sum += band[(width * h + j) as usize] as i32;
        }

        first_new_band[(width * h) as usize] = (sum / cell_count) as u8;

        for w in 1..width {
            if w > radius {
                sum -= band[(width * h + w - radius - 1) as usize] as i32;
                cell_count -= 1;
            }
            if w < width - radius {
                sum += band[(width * h + w + radius) as usize] as i32;
                cell_count += 1;
            }
            first_new_band[(width * h + w) as usize] = (sum / cell_count) as u8;
        }
    }

    for w in 0..width {
        let mut sum: i32 = 0;
        let mut cell_count: i32 = radius + 1;

        for j in 0..(radius + 1) {
            sum += first_new_band[(w + j * width) as usize] as i32;
        }

        second_new_band[w as usize] = (sum / cell_count) as u8;

        for h in 1..height {
            if h > radius {
                sum -= first_new_band[(w + width * (h - radius - 1)) as usize] as i32;
                cell_count -= 1;
            }
            if h < height - radius {
                sum += first_new_band[(w + width * (h + radius)) as usize] as i32;
                cell_count += 1;
            }
            second_new_band[(width * h + w) as usize] = (sum / cell_count) as u8;
        }
    }

    second_new_band
}

/// # Multiplied Box Blur
/// This version calculates the average by *multiplying*
/// instead of *dividing*. It depends on the fact that
///
/// `dividend/divisor = dividend*(1/divisor)`
///
/// As such, we can just compute `1/divisor` ahead of time
/// and multiply the resulting value with the sum (our
/// dividend)
///
/// This results in improved perfomance since division is
/// a more costly operation than multiplication. As such,
/// another area where this code differs from the other
/// box blurs is that the number of pixels used is fixed.
/// Therefore, at the edges of the picture, the outermost
/// pixel is added enough times to make up the same number
/// of pixels
pub fn box_blur_optimized_further(band: &Vec<u8>, width: i32, height: i32, radius: i32) -> Vec<u8> {
    let mut first_new_band: Vec<u8> = vec![];
    let mut second_new_band: Vec<u8> = vec![];

    first_new_band.resize((width * height) as usize, 0 as u8);
    second_new_band.resize((width * height) as usize, 0 as u8);

    let multiplier: f32 = 1.0 / (2.0 * radius as f32 + 1.0);

    for h in 0..height {
        let mut sum: i32 = 0;

        for _ in 1..(radius + 2) {
            sum += band[(width * h) as usize] as i32;
        }

        for j in 1..(radius + 1) {
            sum += band[(width * h + j) as usize] as i32;
        }

        first_new_band[(width * h) as usize] = (sum as f32 * multiplier) as u8;

        for w in 1..width {
            let current_pos: i32 = width * h + w;

            if w > radius + 1 {
                sum -= band[(current_pos - radius - 1) as usize] as i32;
            } else {
                sum -= band[(width * h) as usize] as i32;
            }
            if w < width - radius {
                sum += band[(current_pos + radius) as usize] as i32;
            } else {
                sum += band[(width * h + width - 1) as usize] as i32;
            }
            first_new_band[(current_pos) as usize] = (sum as f32 * multiplier) as u8;
        }
    }

    for w in 0..width {
        let mut sum: i32 = 0;

        for _ in 1..(radius + 2) {
            sum += first_new_band[w as usize] as i32;
        }

        for j in 1..(radius + 1) {
            sum += first_new_band[(w + j * width) as usize] as i32;
        }

        second_new_band[w as usize] = (sum as f32 * multiplier) as u8;

        for h in 1..height {
            if h > radius + 1 {
                sum -= first_new_band[(w + width * (h - radius - 1)) as usize] as i32;
            } else {
                sum -= first_new_band[w as usize] as i32;
            }
            if h < height - radius {
                sum += first_new_band[(w + width * (h + radius)) as usize] as i32;
            } else {
                sum += first_new_band[(w + width * (height - 1)) as usize] as i32;
            }
            second_new_band[(width * h + w) as usize] = (sum as f32 * multiplier) as u8;
        }
    }

    second_new_band
}

/// # Basic Stackblur
/// Uses a simplified version of the stackblur algorithm made [by Mario Klingemann](http://www.quasimondo.com/StackBlurForCanvas/StackBlurDemo.html)
/// to blur the given image band. It is a compromise between the box blur's
/// faster perfomance and the gaussian blur's smoothness.
///
/// Instead of using the `mul_table` and
/// `shg_table` as in the original by Mario Klingemann,
/// this particular function simply finds the sum of the
/// stacks and divides it normally.
pub fn stack_blur(band: &Vec<u8>, width: i32, height: i32, radius: i32) -> Vec<u8> {
    let mut first_new_band: Vec<u8> = vec![];
    let mut second_new_band: Vec<u8> = vec![];

    first_new_band.resize((width * height) as usize, 0 as u8);
    second_new_band.resize((width * height) as usize, 0 as u8);

    for h in 0..height {
        let mut stack: i32 = 0;
        let mut cell_count: i32 = 0;
        let mut incoming_items: i32 = 0;
        let mut outgoing_items: i32 = 0;

        for j in 0..(radius + 1) {
            incoming_items += band[(width * h + j) as usize] as i32;
            stack += (band[(width * h + j) as usize] as i32 * (radius - j + 1)) as i32;
            cell_count += radius - j + 1;
        }

        first_new_band[(width * h) as usize] = (stack / cell_count) as u8;

        for w in 1..width {
            let current_pos = width * h + w;

            if w > radius + 1 {
                outgoing_items -= band[(current_pos - radius - 2) as usize] as i32;
            }
            if w >= width - radius {
                cell_count -= radius + 1 - width + w;
            }
            if w < width - radius {
                incoming_items += band[(current_pos + radius) as usize] as i32;
            }
            if w <= radius {
                cell_count += radius + 1 - w;
            }

            outgoing_items += band[(current_pos - 1) as usize] as i32;
            incoming_items -= band[(current_pos - 1) as usize] as i32;
            stack += incoming_items - outgoing_items;

            first_new_band[(current_pos) as usize] = (stack / cell_count) as u8;
        }
    }

    for w in 0..width {
        let mut stack: i32 = 0;
        let mut cell_count: i32 = 0;
        let mut incoming_items: i32 = 0;
        let mut outgoing_items: i32 = 0;

        for j in 0..(radius + 1) {
            incoming_items += first_new_band[(w + j * width) as usize] as i32;
            stack += (first_new_band[(w + j * width) as usize] as i32 * (radius - j + 1)) as i32;
            cell_count += radius - j + 1;
        }

        second_new_band[w as usize] = (stack / cell_count) as u8;

        for h in 1..height {
            if h > radius + 1 {
                outgoing_items -= first_new_band[(w + width * (h - radius - 2)) as usize] as i32;
            }
            if h >= height - radius {
                cell_count -= radius + 1 - height + h;
            }
            if h < height - radius {
                incoming_items += first_new_band[(w + width * (h + radius)) as usize] as i32;
            }
            if h <= radius {
                cell_count += radius + 1 - h;
            }

            outgoing_items += first_new_band[(w + width * (h - 1)) as usize] as i32;
            incoming_items -= first_new_band[(w + width * (h - 1)) as usize] as i32;
            stack += incoming_items - outgoing_items;

            second_new_band[(w + width * h) as usize] = (stack / cell_count) as u8;
        }
    }

    second_new_band
}

/// # Classic Stackblur
/// This function uses the classic stackblur algorithm
/// [by Mario Klingemann](http://www.quasimondo.com/StackBlurForCanvas/StackBlurDemo.html),
/// including `mul_table`, `shg_table` and all. This
/// delivers the fastest stackblur performance but the biggest
/// drawback is that the blur radius is limited to 255 due to the
/// lengths of `mul_table` and `shg_table`. To go beyond 255, it
/// would require extending them with more values... or by not
/// using `mul_table` and `shg_table` altogether, as in the other
/// stackblur function given here.
///
/// Another weakness of this one is that it gives an overly bright and
/// an overly dark image when radius is 2 and 3, respectively. This is
/// also due to the values in the `mul_table` and `shg_table`.
pub fn stack_blur_optimized(band: &Vec<u8>, width: i32, height: i32, radius: i32) -> Vec<u8> {
    const MUL_TABLE: [u16; 256] = [
        512, 512, 512, 456, 328, 456, 335, 512, 405, 328, 271, 456, 388, 335, 292, 512, 454, 405,
        364, 328, 298, 271, 496, 456, 420, 388, 360, 335, 312, 292, 273, 512, 482, 454, 428, 405,
        383, 364, 345, 328, 312, 298, 284, 271, 259, 496, 475, 456, 437, 420, 404, 388, 374, 360,
        347, 335, 323, 312, 302, 292, 282, 273, 265, 512, 497, 482, 468, 454, 441, 428, 417, 405,
        394, 383, 373, 364, 354, 345, 337, 328, 320, 312, 305, 298, 291, 284, 278, 271, 265, 259,
        507, 496, 485, 475, 465, 456, 446, 437, 428, 420, 412, 404, 396, 388, 381, 374, 367, 360,
        354, 347, 341, 335, 329, 323, 318, 312, 307, 302, 297, 292, 287, 282, 278, 273, 269, 265,
        261, 512, 505, 497, 489, 482, 475, 468, 461, 454, 447, 441, 435, 428, 422, 417, 411, 405,
        399, 394, 389, 383, 378, 373, 368, 364, 359, 354, 350, 345, 341, 337, 332, 328, 324, 320,
        316, 312, 309, 305, 301, 298, 294, 291, 287, 284, 281, 278, 274, 271, 268, 265, 262, 259,
        257, 507, 501, 496, 491, 485, 480, 475, 470, 465, 460, 456, 451, 446, 442, 437, 433, 428,
        424, 420, 416, 412, 408, 404, 400, 396, 392, 388, 385, 381, 377, 374, 370, 367, 363, 360,
        357, 354, 350, 347, 344, 341, 338, 335, 332, 329, 326, 323, 320, 318, 315, 312, 310, 307,
        304, 302, 299, 297, 294, 292, 289, 287, 285, 282, 280, 278, 275, 273, 271, 269, 267, 265,
        263, 261, 259, 257,
    ];

    const SHG_TABLE: [u8; 256] = [
        9, 11, 12, 13, 13, 14, 14, 15, 15, 15, 15, 16, 16, 16, 16, 17, 17, 17, 17, 17, 17, 17, 18,
        18, 18, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 20,
        20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
        23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
        23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24,
        24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
        24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
        24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
        24, 24, 24,
    ];

    let mut first_new_band: Vec<u8> = vec![];
    let mut second_new_band: Vec<u8> = vec![];

    first_new_band.resize((width * height) as usize, 0 as u8);
    second_new_band.resize((width * height) as usize, 0 as u8);

    let mul_sum = MUL_TABLE[radius as usize] as i32;
    let shg_sum = SHG_TABLE[radius as usize] as i32;

    for h in 0..height {
        let mut sum: i32 = 0;
        let mut incoming_items: i32 = band[(width * h) as usize] as i32;
        let mut outgoing_items: i32 = band[(width * h) as usize] as i32 * (radius + 1);

        for j in 1..(radius + 2) {
            sum += band[(width * h) as usize] as i32 * j;
        }

        for j in 1..(radius + 1) {
            incoming_items += band[(width * h + j) as usize] as i32;
            sum += (band[(width * h + j) as usize] as i32 * (radius - j + 1)) as i32;
        }

        first_new_band[(width * h) as usize] = (sum * mul_sum >> shg_sum) as u8;

        for w in 1..width {
            let current_pos = width * h + w;
            if w > radius + 1 {
                outgoing_items -= band[(current_pos - radius - 2) as usize] as i32;
            } else {
                outgoing_items -= band[(width * h) as usize] as i32;
            }

            if w < width - radius {
                incoming_items += band[(current_pos + radius) as usize] as i32;
            } else {
                incoming_items += band[(width * h + width - 1) as usize] as i32;
            }

            outgoing_items += band[(current_pos - 1) as usize] as i32;
            incoming_items -= band[(current_pos - 1) as usize] as i32;
            sum += incoming_items - outgoing_items;
            first_new_band[(current_pos) as usize] = (sum * mul_sum >> shg_sum) as u8;
        }
    }

    for w in 0..width {
        let mut sum: i32 = 0;
        let mut incoming_items: i32 = first_new_band[w as usize] as i32;
        let mut outgoing_items: i32 = first_new_band[w as usize] as i32 * (radius + 1);

        for j in 1..(radius + 2) {
            sum += first_new_band[w as usize] as i32 * j;
        }

        for j in 1..(radius + 1) {
            incoming_items += first_new_band[(w + j * width) as usize] as i32;
            sum += (first_new_band[(w + j * width) as usize] as i32 * (radius - j + 1)) as i32;
        }

        second_new_band[w as usize] = ((sum * mul_sum) >> shg_sum) as u8;

        for h in 1..height {
            if h > radius + 1 {
                outgoing_items -= first_new_band[(w + width * (h - radius - 2)) as usize] as i32;
            } else {
                outgoing_items -= first_new_band[w as usize] as i32;
            }
            if h < height - radius {
                incoming_items += first_new_band[(w + width * (h + radius)) as usize] as i32;
            } else {
                incoming_items += first_new_band[(w + width * (height - 1)) as usize] as i32;
            }

            outgoing_items += first_new_band[(w + width * (h - 1)) as usize] as i32;
            incoming_items -= first_new_band[(w + width * (h - 1)) as usize] as i32;
            sum += incoming_items - outgoing_items;

            second_new_band[(w + width * h) as usize] = ((sum * mul_sum) >> shg_sum) as u8;
        }
    }

    second_new_band
}
