// Day 1480: Sort a million ints in [0, 1e9] without a billion-element array.
// Index by element count, not by value. LSD radix sort (base 256) is O(N).
// For out-of-core data the same idea generalizes to external merge sort.
// Time O(N) (4 passes for 32-bit), Space O(N).
#include <bits/stdc++.h>
using namespace std;

vector<uint32_t> radixSort(vector<uint32_t> out) {
    if (out.empty()) return out;
    vector<uint32_t> tmp(out.size());
    for (int shift = 0; shift < 32; shift += 8) {
        int count[257] = {0};
        for (uint32_t v : out) count[((v >> shift) & 0xFF) + 1]++;
        for (int i = 0; i < 256; ++i) count[i + 1] += count[i];
        for (uint32_t v : out) {
            int d = (v >> shift) & 0xFF;
            tmp[count[d]++] = v;
        }
        swap(out, tmp);
    }
    return out;
}

int main() {
    auto r = radixSort({9, 11, 8, 5, 7, 10});
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << (i ? ", " : "") << r[i];
    cout << "]\n";  // [5, 7, 8, 9, 10, 11]
}
