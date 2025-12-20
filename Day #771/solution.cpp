// Day 771: One-to-one (bijective) character mapping between s1 and s2.
// Track forward and reverse maps; reject conflicts. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isOneToOne(const string& s1, const string& s2) {
    if (s1.size() != s2.size()) return false;
    unordered_map<char, char> fwd, rev;
    for (size_t i = 0; i < s1.size(); i++) {
        char a = s1[i], b = s2[i];
        if (fwd.count(a) && fwd[a] != b) return false;
        if (rev.count(b) && rev[b] != a) return false;
        fwd[a] = b; rev[b] = a;
    }
    return true;
}

int main() {
    cout << boolalpha << isOneToOne("abc", "bcd") << endl; // true
    cout << boolalpha << isOneToOne("foo", "bar") << endl; // false
    return 0;
}
