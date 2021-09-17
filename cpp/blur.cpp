#include <iostream>
#include <cmath>
#include <stdlib.h>
#include <chrono>
#include <sys/time.h>
#include <ctime>
using namespace std;
using namespace chrono;

void printFirstHundred(short unsigned array[], int length);

const int width = 500,
          height = 500,
          radius = 32;
const long size = width * height;

unsigned short band[size];
unsigned short newBand[size];

int main()
{

    for (long i = 0; i < size; i++)
    {
        band[i] = floor(((double)rand() / (double)32767) * 255);
    }

    printFirstHundred(band, size);

    auto start = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();

    for (long i = 0, h = 0; h < height; h++)
    {
        for (int w = 0; w < width; w += 1, i += 1)
        {
            int cellCount = 0;
            long sum = 0;

            for (int m = -radius; m <= radius; m++)
            {
                for (int n = -radius; n <= radius; n++)
                {
                    if (
                        m + h < height && m + h > -1 && n + w < width && n + w > -1)
                    {
                        cellCount++;
                        sum += band[i + (width * m + n)];
                    }
                }
            }

            newBand[i] = round(sum / cellCount);
        }
    }

    auto elapsed = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count() - start;
    double elapsed_secs = (double)elapsed / 1000.0;

    cout << "\n\nTime taken: " << elapsed_secs << " seconds\n";

    printFirstHundred(newBand, size);

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
