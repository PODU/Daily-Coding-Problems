// KMP substring search: build failure/LPS array, then single scan.
// Time: O(N + k), Space: O(k).
#include <bits/stdc++.h>
using namespace std;

// returns start index or -1 (representing False)
int kmpSearch(const string& s, const string& pat) {
    int N = s.size(), k = pat.size();
    if (k == 0) return 0;
    vector<int> lps(k, 0);
    for (int i = 1, len = 0; i < k; ) {
        if (pat[i] == pat[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    for (int i = 0, j = 0; i < N; ) {
        if (s[i] == pat[j]) { i++; j++; if (j == k) return i - k; }
        else if (j) j = lps[j - 1];
        else i++;
    }
    return -1;
}

int main() {
    string s = "abracadabra", pat = "cad";
    int idx = kmpSearch(s, pat);
    if (idx >= 0) cout << idx << "\n"; else cout << "False\n";
    return 0;
}
