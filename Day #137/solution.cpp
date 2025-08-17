// Space-efficient bit array: pack bits into 64-bit words. set/get O(1), space O(size/64).
#include <bits/stdc++.h>
using namespace std;

class BitArray {
    vector<uint64_t> words;
    int n;
public:
    void init(int size) { n = size; words.assign((size + 63) / 64, 0ULL); }
    void set(int i, int val) {
        if (val) words[i >> 6] |= (1ULL << (i & 63));
        else     words[i >> 6] &= ~(1ULL << (i & 63));
    }
    int get(int i) { return (words[i >> 6] >> (i & 63)) & 1ULL; }
};

int main() {
    BitArray b; b.init(10);
    b.set(2, 1); b.set(7, 1); b.set(7, 0); b.set(9, 1);
    cout << b.get(2) << b.get(7) << b.get(9) << b.get(0) << endl; // 1010
    return 0;
}
