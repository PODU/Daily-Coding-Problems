// Day 462: Smallest sparse number (no adjacent set bits) >= N.
// Scan bits low->high, lift each "11" pair upward. Time O(log N), Space O(log N).
#include <iostream>
#include <vector>
using namespace std;

long long nextSparse(long long n) {
    if (n <= 0) return n;
    vector<int> bits;
    for (long long t = n; t > 0; t >>= 1) bits.push_back((int)(t & 1));
    bits.push_back(0); bits.push_back(0); // padding for carries
    int size = (int)bits.size();
    int lastFinal = 0;
    for (int i = 1; i < size - 1; i++) {
        if (bits[i] == 1 && bits[i - 1] == 1 && bits[i + 1] == 0) {
            bits[i + 1] = 1;
            for (int j = i; j >= lastFinal; j--) bits[j] = 0;
            lastFinal = i + 1;
        }
    }
    long long ans = 0;
    for (int i = 0; i < size; i++) if (bits[i]) ans |= (1LL << i);
    return ans;
}

int main() {
    cout << nextSparse(22) << endl; // 32
    cout << nextSparse(21) << endl; // 21 (already sparse)
    return 0;
}
