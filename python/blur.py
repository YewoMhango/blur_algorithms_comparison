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

newBand = []

start = time.time()

i = 0

for h in range(0, HEIGHT):
    for w in range(0, WIDTH):
        cellCount = 0
        sumOfNumbers = 0

        for m in range(-RADIUS, RADIUS + 1):
            for n in range(-RADIUS, RADIUS + 1):
                if not (m + h >= HEIGHT or m + h <= -1 or n + w >= WIDTH or n + w <= -1):
                    cellCount += 1
                    sumOfNumbers += band[i + (WIDTH * m + n)]

        newBand.append(round(sumOfNumbers / cellCount))

elapsed = time.time() - start

print("Time taken:", elapsed, "seconds")
print(newBand[0: 100], "...", SIZE - 100, "more items")
