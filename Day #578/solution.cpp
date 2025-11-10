// Bijective char mapping: maintain s1->s2 (consistent) and s2->s1 (injective) maps. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

bool bijective(const string &s1, const string &s2) {
    if (s1.size() != s2.size()) return false;
    map<char, char> fwd, bwd;
    for (size_t i = 0; i < s1.size(); i++) {
        char a = s1[i], b = s2[i];
        if (fwd.count(a) && fwd[a] != b) return false;
        if (bwd.count(b) && bwd[b] != a) return false;
        fwd[a] = b;
        bwd[b] = a;
    }
    return true;
}

int main() {
    if (bijective("abc", "bcd"))
        cout << "true (map a to b, b to c, and c to d)\n";
    if (!bijective("foo", "bar"))
        cout << "false (the o cannot map to two characters)\n";
    return 0;
}
