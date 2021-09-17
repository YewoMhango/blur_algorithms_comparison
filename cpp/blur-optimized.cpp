#include <iostream>
#include <cmath>
#include <stdlib.h>
#include <chrono>
#include <sys/time.h>
#include <ctime>
using namespace std;
using namespace chrono;

const int width = 1920,
          height = 1200,
          radius = 15;
const long size = width * height;

short unsigned firstNewBand[size];
short unsigned band[size];

void printFirstHundred(short unsigned array[], int length);

int main()
{
    for (long i = 0; i < size; i++)
    {
        band[i] = floor(((double)rand() / (double)32767) * 255);
    }

    printFirstHundred(band, size);

    auto start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();

    for (int h = 0; h < height; h++)
    {
        long sum = 0;
        int cellCount = radius + 1;

        for (int j = 0; j <= radius; j++)
        {
            sum += band[width * h + j];
        }

        firstNewBand[width * h] = round(sum / cellCount);

        for (int w = 1; w < width; w++)
        {
            if (w > radius)
            {
                sum -= band[width * h + w - radius - 1];
                cellCount--;
            }
            if (w < width - radius)
            {
                sum += band[width * h + w + radius];
                cellCount++;
            }
            firstNewBand[width * h + w] = round(sum / cellCount);
        }
    }

    for (int w = 0; w < width; w++)
    {
        long sum = 0;
        int cellCount = radius + 1;

        for (int j = 0; j <= radius; j++)
        {
            sum += firstNewBand[w + j * width];
        }

        band[w] = round(sum / cellCount);

        for (int h = 1; h < height; h++)
        {
            if (h > radius)
            {
                sum -= firstNewBand[w + width * (h - radius - 1)];
                cellCount--;
            }
            if (h < height - radius)
            {
                sum += firstNewBand[w + width * (h + radius)];
                cellCount++;
            }
            band[w + width * h] = round(sum / cellCount);
        }
    }

    auto end = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count() - start;
    double elapsed_secs = (double)end / 1000.0;

    cout << "\n\nTime taken: " << elapsed_secs << " seconds\n";

    printFirstHundred(band, size);

    return 0;
}

void printFirstHundred(short unsigned array[], int length)
{
    cout << "\n[ " << array[0];

    for (int i = 1; i < min(100, length); i++)
    {
        cout << ", " << array[i];
    }

    cout << " ... " << length - 100 << " more items ]\n";
}
