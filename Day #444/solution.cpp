// Day 444: KMP string matching in O(N + k) time, O(k) space (beats O(N*k)).
// Returns the start index of the first match, or -1 (False) if absent.
#include <bits/stdc++.h>
using namespace std;

vector<int> buildLPS(const string& p) {
    int k = p.size();
    vector<int> lps(k, 0);
    int len = 0;
    for (int i = 1; i < k; ) {
        if (p[i] == p[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    return lps;
}

int kmpSearch(const string& text, const string& pat) {
    if (pat.empty()) return 0;
    vector<int> lps = buildLPS(pat);
    int i = 0, j = 0, n = text.size(), k = pat.size();
    while (i < n) {
        if (text[i] == pat[j]) { i++; j++; if (j == k) return i - k; }
        else if (j) j = lps[j - 1];
        else i++;
    }
    return -1; // not found (False)
}

int main() {
    string text = "abxabcabcaby", pat = "abcaby";
    int idx = kmpSearch(text, pat);
    if (idx == -1) cout << "False\n";
    else cout << idx << "\n"; // 6
    return 0;
}
