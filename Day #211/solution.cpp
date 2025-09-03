// Day 211: All occurrences of pattern in string via KMP.
// Approach: Knuth-Morris-Pratt with LPS failure function. Time O(n+m), Space O(m).
#include <bits/stdc++.h>
using namespace std;

vector<int> findOccurrences(const string& s, const string& p) {
    vector<int> res;
    int m = p.size(), n = s.size();
    if (m == 0 || m > n) return res;
    vector<int> lps(m, 0);
    for (int i = 1, len = 0; i < m;) {
        if (p[i] == p[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    for (int i = 0, j = 0; i < n;) {
        if (s[i] == p[j]) { i++; j++; if (j == m) { res.push_back(i - m); j = lps[j - 1]; } }
        else if (j) j = lps[j - 1];
        else i++;
    }
    return res;
}

int main() {
    string s = "abracadabra", p = "abr";
    auto r = findOccurrences(s, p);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
