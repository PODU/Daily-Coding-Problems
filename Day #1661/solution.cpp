// Bloom filter: fixed 1000-bit array, 3 hashes via double hashing. add/check.
// add O(k), check O(k); Space O(m bits). check may false-positive, never false-negative.
#include <bits/stdc++.h>
using namespace std;
struct Bloom {
    static const int M = 1000, K = 3;
    bitset<M> bits;
    unsigned long h1(const string& s) { unsigned long h = 5381; for (char c : s) h = h * 33 + (unsigned char)c; return h; }
    unsigned long h2(const string& s) { unsigned long h = 0; for (char c : s) h = h * 131 + (unsigned char)c; return h; }
    void add(const string& s) {
        unsigned long a = h1(s), b = h2(s);
        for (int i = 0; i < K; i++) bits[(a + (unsigned long)i * b) % M] = 1;
    }
    bool check(const string& s) {
        unsigned long a = h1(s), b = h2(s);
        for (int i = 0; i < K; i++) if (!bits[(a + (unsigned long)i * b) % M]) return false;
        return true;
    }
};
int main() {
    Bloom bf;
    bf.add("apple"); bf.add("banana"); bf.add("cat");
    cout << boolalpha;
    cout << "check apple: " << bf.check("apple") << "\n";
    cout << "check banana: " << bf.check("banana") << "\n";
    cout << "check cat: " << bf.check("cat") << "\n";
    cout << "check dog: " << bf.check("dog") << "\n";
    return 0;
}
