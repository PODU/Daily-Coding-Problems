// Day 1212: Space-efficient bit array backed by 64-bit words.
// Pack bits into words; set/get use word index and bit offset. Time O(1) per op, Space O(size/64).
#include <bits/stdc++.h>
using namespace std;

class BitArray {
    vector<uint64_t> words;
    int n;
public:
    BitArray(int size): words((size + 63) / 64, 0), n(size) {}
    void set(int i, int val) {
        if (val) words[i >> 6] |= (1ULL << (i & 63));
        else     words[i >> 6] &= ~(1ULL << (i & 63));
    }
    int get(int i) const { return (words[i >> 6] >> (i & 63)) & 1ULL; }
};

int main() {
    BitArray b(10);
    b.set(2, 1); b.set(7, 1); b.set(2, 0);
    cout << b.get(2) << " " << b.get(7) << " " << b.get(0) << "\n"; // 0 1 0
    return 0;
}
