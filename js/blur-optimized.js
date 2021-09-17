const width = 500,
  height = 500,
  size = width * height,
  radius = 32;

let band = [];

for (let i = 0; i < size; i++) {
  band.push(Math.floor(Math.random() * 255));
}

console.log("Original:", band);

function main() {
  let start = Date.now();

  let firstNewBand = [];

  for (let h = 0; h < height; h++) {
    let sum = 0;
    let cellCount = radius + 1;

    for (let j = 0; j <= radius; j++) {
      sum += band[width * h + j];
    }

    firstNewBand.push(Math.round(sum / cellCount));

    for (let w = 1; w < width; w++) {
      if (w > radius) {
        sum -= band[width * h + w - radius - 1];
        cellCount--;
      }
      if (w < width - radius) {
        sum += band[width * h + w + radius];
        cellCount++;
      }
      firstNewBand.push(Math.round(sum / cellCount));
    }
  }

  let secondNewBand = [];

  for (let w = 0; w < width; w++) {
    let sum = 0;
    let cellCount = radius + 1;

    for (let j = 0; j <= radius; j++) {
      sum += firstNewBand[w + j * width];
    }

    secondNewBand[w] = Math.round(sum / cellCount);

    for (let h = 1; h < height; h++) {
      if (h > radius) {
        sum -= firstNewBand[w + width * (h - radius - 1)];
        cellCount--;
      }
      if (h < height - radius) {
        sum += firstNewBand[w + width * (h + radius)];
        cellCount++;
      }
      secondNewBand[w + width * h] = Math.round(sum / cellCount);
    }
  }

  let elapsed = (Date.now() - start) / 1000;

  console.log("Time taken:", elapsed, "seconds");
  console.log("Output:", secondNewBand);
}

for (let i = 0; i < 10; i++) {
  main();
}
