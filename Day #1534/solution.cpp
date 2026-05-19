// Space-efficient bit array packing 32 bits per word.
// init(size), set(i,val), get(i): each O(1); space O(size/32) words.
#include <bits/stdc++.h>
using namespace std;

class BitArray {
    vector<uint32_t> words;
    int n;
public:
    void init(int size) {
        n = size;
        words.assign((size + 31) / 32, 0);
    }
    void set(int i, int val) {
        if (val) words[i >> 5] |= (1u << (i & 31));
        else     words[i >> 5] &= ~(1u << (i & 31));
    }
    int get(int i) {
        return (words[i >> 5] >> (i & 31)) & 1u;
    }
};

int main() {
    BitArray b;
    b.init(10);
    b.set(1, 1);
    b.set(4, 1);
    b.set(4, 0);
    b.set(9, 1);
    cout << b.get(1) << " " << b.get(4) << " " << b.get(9) << " " << b.get(0) << "\n";
    // expected: 1 0 1 0
    return 0;
}
