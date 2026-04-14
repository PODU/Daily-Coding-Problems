// Reservoir sampling (size 1): i-th element (1-indexed) replaces pick with prob 1/i. O(1) space.
// Demo uses a portable 64-bit LCG seeded with 1 so output is deterministic across languages -> 7.
#include <iostream>
#include <vector>
#include <cstdint>
using namespace std;

int main() {
    vector<int> stream = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    const uint64_t A = 6364136223846793005ULL, C = 1442695040888963407ULL;
    uint64_t state = 1ULL; // fixed seed

    int pick = 0;
    for (uint64_t i = 1; i <= stream.size(); ++i) {
        state = state * A + C;            // advance LCG (mod 2^64 via overflow)
        if (state % i == 0)               // replace with probability 1/i
            pick = stream[i - 1];
    }
    cout << "Selected: " << pick << "\n";
    return 0;
}
