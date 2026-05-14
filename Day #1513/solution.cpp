// KMP pattern matching: build failure (LPS) array, then scan once collecting all match starts.
// Time: O(N+k), Space: O(k).
#include <bits/stdc++.h>
using namespace std;

vector<int> findAll(const string& s, const string& p) {
    int n = s.size(), k = p.size();
    vector<int> res;
    if (k == 0 || k > n) return res;
    vector<int> lps(k, 0);
    for (int i = 1, len = 0; i < k;) {
        if (p[i] == p[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    for (int i = 0, j = 0; i < n;) {
        if (s[i] == p[j]) { i++; j++; if (j == k) { res.push_back(i - k); j = lps[j - 1]; } }
        else if (j) j = lps[j - 1];
        else i++;
    }
    return res;
}

int main() {
    vector<int> r = findAll("abracadabra", "abr");
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
