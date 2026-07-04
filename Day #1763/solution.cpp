// Day 1763: Sort ~1e6 ints in range [0,1e9]. The MILLION elements fit in memory
// (~4MB); only the billion VALUE range is large. Use LSD radix sort (base 256,
// 4 passes) — O(n) time, O(n) space, optimal for fixed-width integers.
// If even the million don't fit, fall back to external merge sort (chunk-sort to
// disk, then k-way merge).
#include <bits/stdc++.h>
using namespace std;

void radixSort(vector<uint32_t>& a) {
    vector<uint32_t> tmp(a.size());
    for (int shift = 0; shift < 32; shift += 8) {
        size_t count[256] = {0};
        for (uint32_t x : a) count[(x >> shift) & 0xFF]++;
        size_t sum = 0;
        for (int i = 0; i < 256; ++i) { size_t c = count[i]; count[i] = sum; sum += c; }
        for (uint32_t x : a) tmp[count[(x >> shift) & 0xFF]++] = x;
        a.swap(tmp);
    }
}

int main() {
    vector<uint32_t> a = {999999999u, 0u, 123456789u, 42u, 1000000000u, 7u, 500000000u};
    radixSort(a);
    cout << "[";
    for (size_t i = 0; i < a.size(); ++i) cout << a[i] << (i + 1 < a.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
