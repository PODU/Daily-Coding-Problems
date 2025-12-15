// Bloom filter: fixed bit array, k hash functions via double hashing.
// check() may report false positives but never false negatives.
// Time: O(k) per add/check, Space: O(m bits).
#include <bits/stdc++.h>
using namespace std;

class BloomFilter {
    vector<bool> bits;
    int m, k;
    pair<unsigned long long, unsigned long long> hashes(const string& s) const {
        unsigned long long h1 = 1469598103934665603ULL; // FNV-1a
        for(char c : s){ h1 ^= (unsigned char)c; h1 *= 1099511628211ULL; }
        unsigned long long h2 = 5381;
        for(char c : s){ h2 = ((h2 << 5) + h2) + (unsigned char)c; } // djb2
        return {h1, h2};
    }
public:
    BloomFilter(int size=1000, int numHashes=4): bits(size,false), m(size), k(numHashes) {}
    void add(const string& s){
        auto h = hashes(s);
        unsigned long long h1 = h.first, h2 = h.second;
        for(int i=0;i<k;i++) bits[(h1 + (unsigned long long)i*h2) % m] = true;
    }
    bool check(const string& s) const {
        auto h = hashes(s);
        unsigned long long h1 = h.first, h2 = h.second;
        for(int i=0;i<k;i++) if(!bits[(h1 + (unsigned long long)i*h2) % m]) return false;
        return true;
    }
};

int main(){
    BloomFilter bf;
    bf.add("apple"); bf.add("banana");
    cout << boolalpha;
    cout << "apple: "  << bf.check("apple")  << "\n"; // true
    cout << "banana: " << bf.check("banana") << "\n"; // true
    cout << "cherry: " << bf.check("cherry") << "\n"; // false
    return 0;
}
