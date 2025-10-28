// Approximate median: sample a fixed, sublinear subset (size independent of N),
// return the sample's median -> lands in the central half [N/4, 3N/4] w.h.p.
// Sampling+median: O(s log s), sublinear in N. (Rank shown only for the demo.)
#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdint>

static uint64_t state = 0x0123456789ABCDEFULL; // fixed seed -> deterministic

uint64_t next_rand() {
    state = state * 6364136223846793005ULL + 1442695040888963407ULL;
    return state >> 33; // top 31 bits
}

int main() {
    const int N = 1000;
    const int SAMPLE_SIZE = 99; // fixed, independent of N (sublinear)

    std::vector<int> data(N);
    for (int i = 0; i < N; ++i) data[i] = i + 1;
    // Fisher-Yates shuffle
    for (int i = N - 1; i > 0; --i) {
        int j = (int)(next_rand() % (uint64_t)(i + 1));
        std::swap(data[i], data[j]);
    }

    // Random sample (with replacement)
    std::vector<int> sample;
    sample.reserve(SAMPLE_SIZE);
    for (int i = 0; i < SAMPLE_SIZE; ++i) {
        int idx = (int)(next_rand() % (uint64_t)N);
        sample.push_back(data[idx]);
    }
    std::sort(sample.begin(), sample.end());
    int median = sample[SAMPLE_SIZE / 2];

    // actual rank (count of values <= median); demo-only O(N)
    int rank = 0;
    for (int v : data) if (v <= median) ++rank;

    std::cout << "Approximate median: " << median << std::endl;
    std::cout << "Rank: " << rank << " (acceptable range: " << (N / 4)
              << " to " << (3 * N / 4) << ")" << std::endl;
    return 0;
}
