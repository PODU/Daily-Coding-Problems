// Day 574: Space-efficient bit array backed by 32-bit words.
// set/get are O(1); storage is ceil(size/32) words.
#include <iostream>
#include <vector>
#include <stdexcept>
using namespace std;

class BitArray {
    vector<uint32_t> words;
    int n;
public:
    explicit BitArray(int size) : words((size + 31) / 32, 0), n(size) {}
    void set(int i, int val) {
        if (i < 0 || i >= n) throw out_of_range("index");
        if (val) words[i >> 5] |=  (1u << (i & 31));
        else     words[i >> 5] &= ~(1u << (i & 31));
    }
    int get(int i) const {
        if (i < 0 || i >= n) throw out_of_range("index");
        return (words[i >> 5] >> (i & 31)) & 1u;
    }
};

int main() {
    BitArray b(8);
    b.set(0, 1);
    b.set(3, 1);
    cout << "get(0) = " << b.get(0) << "\n"; // 1
    cout << "get(1) = " << b.get(1) << "\n"; // 0
    cout << "get(3) = " << b.get(3) << "\n"; // 1
    b.set(3, 0);
    cout << "get(3) = " << b.get(3) << "\n"; // 0
    return 0;
}
