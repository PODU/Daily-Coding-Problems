// Day 217: Smallest sparse number (no two adjacent set bits) >= N.
// Approach: scan bits low->high; on adjacent 11 with a 0 above, carry up and clear lower bits.
// Time O(bits) ~ O(log N), much faster than O(N log N). Space O(log N).
#include <bits/stdc++.h>
using namespace std;

long long nextSparse(long long n) {
    if (n <= 0) return 0;
    vector<int> bits;
    long long x = n;
    while (x) { bits.push_back(x & 1); x >>= 1; }
    bits.push_back(0); bits.push_back(0); // headroom for carries
    int sz = bits.size();
    int lastFinal = 0;
    for (int i = 1; i < sz - 1; i++) {
        if (bits[i] == 1 && bits[i - 1] == 1 && bits[i + 1] == 0) {
            bits[i + 1] = 1;
            for (int j = i; j >= lastFinal; j--) bits[j] = 0;
            lastFinal = i + 1;
        }
    }
    long long res = 0;
    for (int i = sz - 1; i >= 0; i--) res = (res << 1) | bits[i];
    return res;
}

int main() {
    cout << nextSparse(22) << endl; // 22 = 10110 -> 32 (100000)
    cout << nextSparse(21) << endl; // 21 = 10101 already sparse -> 21
    return 0;
}
