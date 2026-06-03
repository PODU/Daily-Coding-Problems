// Bijective char mapping s1->s2: lengths equal + consistent forward map + injective (no two s1 chars share an s2 char).
// Time O(n), Space O(1) (fixed 256-size maps).
#include <bits/stdc++.h>
using namespace std;

bool bijectiveMap(const string& s1, const string& s2) {
    if (s1.size() != s2.size()) return false;
    int fwd[256], rev[256];
    fill(fwd, fwd + 256, -1);
    fill(rev, rev + 256, -1);
    for (size_t i = 0; i < s1.size(); ++i) {
        unsigned char a = s1[i], b = s2[i];
        if (fwd[a] == -1 && rev[b] == -1) { fwd[a] = b; rev[b] = a; }
        else if (fwd[a] != b || rev[b] != a) return false;
    }
    return true;
}

int main() {
    cout << (bijectiveMap("abc", "bcd") ? "true" : "false") << "\n";
    cout << (bijectiveMap("foo", "bar") ? "true" : "false") << "\n";
    return 0;
}
