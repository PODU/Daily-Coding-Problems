// Day 421: LSD radix sort (base-256). Sorts n ints in O(n*w) time, O(n) space.
// w = 4 byte-passes for 32-bit values; no billion-element counting array needed.
#include <bits/stdc++.h>
using namespace std;

void radixSort(vector<uint32_t>& a) {
    int n = a.size();
    vector<uint32_t> buf(n);
    for (int shift = 0; shift < 32; shift += 8) {
        int cnt[257] = {0};
        for (int i = 0; i < n; i++) cnt[((a[i] >> shift) & 0xFF) + 1]++;
        for (int i = 0; i < 256; i++) cnt[i + 1] += cnt[i];
        for (int i = 0; i < n; i++) buf[cnt[(a[i] >> shift) & 0xFF]++] = a[i];
        swap(a, buf);
    }
}

int main() {
    vector<uint32_t> a = {5, 3, 1000000000, 0, 42, 7, 42};
    radixSort(a);
    cout << "Sorted: [";
    for (size_t i = 0; i < a.size(); i++) {
        cout << a[i];
        if (i + 1 < a.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
