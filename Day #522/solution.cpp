// KMP: build prefix-function for pattern, scan text. O(n+m) time, O(m) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> kmpSearch(const string& s, const string& p) {
    int m = p.size();
    vector<int> lps(m, 0);
    for (int i = 1, len = 0; i < m;) {
        if (p[i] == p[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    vector<int> res;
    for (int i = 0, j = 0; i < (int)s.size();) {
        if (s[i] == p[j]) { i++; j++; if (j == m) { res.push_back(i - m); j = lps[j - 1]; } }
        else if (j) j = lps[j - 1];
        else i++;
    }
    return res;
}

int main() {
    vector<int> r = kmpSearch("abracadabra", "abr");
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
