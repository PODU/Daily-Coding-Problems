// Bloom filter: fixed bit array (1000 bits) + k=3 hashes via double hashing.
// add/check are O(k); space O(m) bits. check has false positives, no false negatives.
#include <bits/stdc++.h>
using namespace std;

class BloomFilter {
    static const int SIZE = 1000;
    static const int K = 3;
    vector<bool> bits;

    pair<size_t,size_t> baseHashes(const string& s) const {
        size_t h1 = std::hash<string>{}(s);
        size_t h2 = std::hash<string>{}(s + "salt");
        return {h1, h2};
    }
public:
    BloomFilter() : bits(SIZE, false) {}

    void add(const string& v) {
        pair<size_t,size_t> h = baseHashes(v);
        for (int i = 0; i < K; ++i)
            bits[(h.first + (size_t)i * h.second) % SIZE] = true;
    }

    bool check(const string& v) const {
        pair<size_t,size_t> h = baseHashes(v);
        for (int i = 0; i < K; ++i)
            if (!bits[(h.first + (size_t)i * h.second) % SIZE]) return false;
        return true;
    }
};

int main() {
    BloomFilter bf;
    vector<string> added = {"apple", "banana", "cherry"};
    for (auto& s : added) bf.add(s);

    cout << "Added values (expect all true):\n";
    for (auto& s : added) cout << "  check(" << s << ") = " << (bf.check(s) ? "true" : "false") << "\n";

    cout << "Non-added values (expect mostly false):\n";
    for (auto& s : {"date", "elderberry", "fig", "grape"})
        cout << "  check(" << s << ") = " << (bf.check(s) ? "true" : "false") << "\n";
    return 0;
}
