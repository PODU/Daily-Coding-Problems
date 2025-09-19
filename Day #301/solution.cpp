// Day 301: Bloom filter - fixed-size probabilistic set. No false negatives.
// add/check O(k); space O(m) bits.
#include <bits/stdc++.h>
using namespace std;
struct BloomFilter {
    vector<bool> bits; int k;
    BloomFilter(int m, int k_) : bits(m, false), k(k_) {}
    size_t h(const string& s, int i) const {
        size_t h1 = hash<string>{}(s);
        size_t h2 = hash<string>{}(s + "#salt");
        return (h1 + (size_t)i * h2) % bits.size();
    }
    void add(const string& v) { for (int i = 0; i < k; i++) bits[h(v, i)] = true; }
    bool check(const string& v) const {
        for (int i = 0; i < k; i++) if (!bits[h(v, i)]) return false;
        return true;
    }
};
int main() {
    BloomFilter bf(1000, 4);
    bf.add("apple"); bf.add("banana");
    cout << boolalpha;
    cout << bf.check("apple") << "\n";   // true
    cout << bf.check("banana") << "\n";  // true
    cout << bf.check("cherry") << "\n";  // false (almost certainly)
}
