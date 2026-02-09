// LSD radix sort (base 256, 4 passes over 32-bit ints). O(N*d)~O(N) time, O(N) space.
// Avoids a 1e9-size counting array (infeasible); memory scales with N, not value range.
// If even N elements don't fit in RAM, fall back to external merge sort.
#include <bits/stdc++.h>
using namespace std;

void radixSort(vector<uint32_t>& a) {
    size_t n = a.size();
    vector<uint32_t> buf(n);
    for (int shift = 0; shift < 32; shift += 8) {
        size_t count[256] = {0};
        for (uint32_t v : a) count[(v >> shift) & 0xFF]++;
        size_t sum = 0;
        for (int i = 0; i < 256; i++) { size_t c = count[i]; count[i] = sum; sum += c; }
        for (uint32_t v : a) buf[count[(v >> shift) & 0xFF]++] = v;
        a.swap(buf);
    }
}

int main() {
    vector<uint32_t> a = {829, 3, 1000000000u, 42, 17, 999, 256, 0, 524287, 42};
    radixSort(a);
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? " " : "\n");
    return 0;
}
