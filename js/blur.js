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
  let newBand = [];

  let start = Date.now();

  for (let i = 0, h = 0; h < height; h++) {
    for (let w = 0; w < width; w += 1, i += 1) {
      let cellCount = 0,
        sum = 0;

      for (let m = -radius; m <= radius; m++) {
        for (let n = -radius; n <= radius; n++) {
          if (m + h < height && m + h > -1 && n + w < width && n + w > -1) {
            cellCount++;
            sum += band[i + (width * m + n)];
          }
        }
      }

      newBand.push(Math.round(sum / cellCount));
    }
  }

  let elapsed = (Date.now() - start) / 1000;

  console.log("Time taken:", elapsed, "seconds");
  console.log("Output:", newBand);
}

for (let i = 0; i < 10; i++) {
  main();
}
