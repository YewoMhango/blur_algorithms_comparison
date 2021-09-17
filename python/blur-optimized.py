import math
import random
import time

WIDTH = 500
HEIGHT = 500
SIZE = WIDTH * HEIGHT
RADIUS = 32

band = []

for i in range(0, SIZE):
    band.append(math.floor(random.randint(0, 255)))

print(band[0: 100], "...", SIZE - 100, "more items")

start = time.time()

firstNewBand = []

for h in range(0, HEIGHT):
    sumOfNumbers = 0
    cellCount = RADIUS + 1

    for j in range(0, RADIUS + 1):
        sumOfNumbers += band[WIDTH * h + j]

    firstNewBand.append(round(sumOfNumbers / cellCount))

    for w in range(0, WIDTH):
        if w > RADIUS:
            sumOfNumbers -= band[WIDTH * h + w - RADIUS - 1]
            cellCount -= 1
        if w < WIDTH - RADIUS:
            sumOfNumbers += band[WIDTH * h + w + RADIUS]
            cellCount += 1

        firstNewBand.append(round(sumOfNumbers / cellCount))


secondNewBand = list(range(0, SIZE))

for w in range(0, WIDTH):
    sumOfNumbers = 0
    cellCount = RADIUS + 1

    for j in range(0, RADIUS + 1):
        sumOfNumbers += firstNewBand[w + j * WIDTH]

    secondNewBand[w] = round(sumOfNumbers / cellCount)

    for h in range(0, HEIGHT):
        if h > RADIUS:
            sumOfNumbers -= firstNewBand[w + WIDTH * (h - RADIUS - 1)]
            cellCount -= 1

        if h < HEIGHT - RADIUS:
            sumOfNumbers += firstNewBand[w + WIDTH * (h + RADIUS)]
            cellCount += 1

        secondNewBand[w + WIDTH * h] = round(sumOfNumbers / cellCount)

elapsed = time.time() - start

print("Time taken:", elapsed, "seconds")
print(secondNewBand[0: 100], "...", SIZE - 100, "more items")
