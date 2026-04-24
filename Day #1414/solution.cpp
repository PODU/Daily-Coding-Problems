// Day 1414: does a one-to-one (bijective) char mapping s1 -> s2 exist?
// Approach: enforce a consistent forward map AND injective (no two srcs share a target). O(n).
#include <bits/stdc++.h>
using namespace std;

bool canMap(const string& s1, const string& s2) {
    if (s1.size() != s2.size()) return false;
    unordered_map<char,char> fwd, rev;
    for (size_t i = 0; i < s1.size(); ++i) {
        char a = s1[i], b = s2[i];
        if (fwd.count(a) && fwd[a] != b) return false;
        if (rev.count(b) && rev[b] != a) return false;
        fwd[a] = b; rev[b] = a;
    }
    return true;
}

int main() {
    cout << (canMap("abc", "bcd") ? "true" : "false") << "\n"; // true
    cout << (canMap("foo", "bar") ? "true" : "false") << "\n"; // false
    return 0;
}
