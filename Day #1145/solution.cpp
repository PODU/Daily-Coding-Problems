// Day 1145: Bloom filter - fixed-size bit array, k hashes.
// add/check O(k); check has false positives but never false negatives.
#include <bits/stdc++.h>
using namespace std;

class BloomFilter {
    vector<bool> bits;
    int k;
    size_t hashI(const string& s, int i) const {
        size_t h = 1469598103934665603ULL ^ (i + 1);
        for (char c : s) { h ^= (unsigned char)c; h *= 1099511628211ULL; }
        return h % bits.size();
    }
public:
    BloomFilter(size_t m, int k_) : bits(m, false), k(k_) {}
    void add(const string& v) { for (int i = 0; i < k; ++i) bits[hashI(v, i)] = true; }
    bool check(const string& v) const {
        for (int i = 0; i < k; ++i) if (!bits[hashI(v, i)]) return false;
        return true;
    }
};

int main() {
    BloomFilter bf(1000, 4);
    bf.add("apple"); bf.add("banana");
    cout << boolalpha << bf.check("apple") << " " << bf.check("banana")
         << " " << bf.check("cherry") << "\n"; // true true false (likely)
    return 0;
}
