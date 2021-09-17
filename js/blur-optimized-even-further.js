const width = 1920,
  height = 1080,
  size = width * height,
  radius = 32;

let band = new Uint8ClampedArray(size);

for (let i = 0; i < size; i++) {
  band[i] = Math.floor(Math.random() * 255);
}

console.log("Original:", band);

function main() {
  let firstNewBand = new Uint8ClampedArray(size);
  let secondNewBand = new Uint8ClampedArray(size);

  let start = Date.now();

  const multiplier = 1 / (2 * radius + 1);

  for (let h = 0; h < height; h++) {
    let sum = 0;

    for (let j = 1; j <= radius + 1; j++) {
      sum += band[width * h];
    }

    for (let j = 1; j <= radius; j++) {
      sum += band[width * h + j];
    }

    firstNewBand[width * h] = sum * multiplier;

    for (let w = 1; w < width; w++) {
      let currentPos = width * h + w;

      if (w > radius + 1) {
        sum -= band[currentPos - radius - 2];
      } else {
        sum -= band[width * h];
      }

      if (w < width - radius) {
        sum += band[currentPos + radius];
      } else {
        sum += band[width * h + width - 1];
      }

      firstNewBand[currentPos] = sum * multiplier;
    }
  }

  for (let w = 0; w < width; w++) {
    let sum = 0;

    for (let j = 1; j <= radius + 1; j++) {
      sum += firstNewBand[w];
    }

    for (let j = 1; j <= radius; j++) {
      sum += firstNewBand[w + j * width];
    }

    secondNewBand[w] = sum * multiplier;

    for (let h = 1; h < height; h++) {
      if (h > radius + 1) {
        sum -= firstNewBand[w + width * (h - radius - 1)];
      } else {
        sum -= firstNewBand[w];
      }

      if (h < height - radius) {
        sum += firstNewBand[w + width * (h + radius)];
      } else {
        sum += firstNewBand[w + width * (height - 1)];
      }

      secondNewBand[w + width * h] = sum * multiplier;
    }
  }

  let end = Date.now();
  let elapsed = (end - start) / 1000;

  console.log("Time taken:", elapsed, "seconds");
  console.log("Output:", secondNewBand);
}

for (let i = 0; i < 10; i++) {
  main();
}
