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
const float multiplier = 1.0 / (2.0 * radius + 1.0);

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

    // Start of processing ==================

    for (int h = 0; h < height; h++)
    {
        double sum = 0;

        for (int j = 1; j <= radius + 1; j++)
        {
            sum += band[width * h];
        }

        for (int j = 1; j <= radius; j++)
        {
            sum += band[width * h + j];
        }

        firstNewBand[width * h] = round(sum * multiplier);

        for (int w = 1; w < width; w++)
        {
            int currentPos = width * h + w;

            if (w > radius)
            {
                sum -= band[currentPos - radius - 1];
            }
            else
            {
                sum -= band[width * h];
            }

            if (w < width - radius)
            {
                sum += band[currentPos + radius];
            }
            else
            {
                sum += band[width * h + width - 1];
            }

            firstNewBand[currentPos] = round(sum * multiplier);
        }
    }

    for (int w = 0; w < width; w++)
    {
        double sum = 0;
        int cellCount = radius + 1;

        for (int j = 1; j <= radius + 1; j++)
        {
            sum += firstNewBand[w];
        }

        for (int j = 1; j <= radius; j++)
        {
            sum += firstNewBand[w + j * width];
        }

        band[w] = round(sum * multiplier);

        for (int h = 1; h < height; h++)
        {
            if (h > radius + 1)
            {
                sum -= firstNewBand[w + width * (h - radius - 1)];
            }
            else
            {
                sum -= firstNewBand[w];
            }

            if (h < height - radius)
            {
                sum += firstNewBand[w + width * (h + radius)];
            }
            else
            {
                sum += firstNewBand[w + width * (height - 1)];
            }

            band[w + width * h] = round(sum * multiplier);
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
