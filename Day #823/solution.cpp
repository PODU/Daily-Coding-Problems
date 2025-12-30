// Space-efficient bit array using 32-bit words; index>>5 picks word, 1<<(index&31) picks bit.
// Time: O(1) per op, Space: O(n/32 words).
#include <bits/stdc++.h>
using namespace std;

struct BitArray {
    vector<uint32_t> words;
    BitArray(int size) : words((size + 31) >> 5, 0) {}
    void set(int i, int val) {
        if (val) words[i >> 5] |= (1u << (i & 31));
        else     words[i >> 5] &= ~(1u << (i & 31));
    }
    int get(int i) const { return (words[i >> 5] >> (i & 31)) & 1u; }
};

int main() {
    BitArray ba(16);
    ba.set(0, 1);
    ba.set(5, 1);
    ba.set(15, 1);
    cout << "get(0)=" << ba.get(0) << "\n";
    cout << "get(1)=" << ba.get(1) << "\n";
    cout << "get(5)=" << ba.get(5) << "\n";
    cout << "get(15)=" << ba.get(15) << "\n";
    return 0;
}
