// KMP: build LPS, scan once, record every full match start. Time O(N+k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

vector<int> findAll(const string& s, const string& pat) {
    vector<int> res;
    int N = s.size(), k = pat.size();
    if (k == 0 || k > N) return res;
    vector<int> lps(k, 0);
    for (int i = 1, len = 0; i < k; ) {
        if (pat[i] == pat[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    for (int i = 0, j = 0; i < N; ) {
        if (s[i] == pat[j]) {
            i++; j++;
            if (j == k) { res.push_back(i - k); j = lps[j - 1]; }
        } else if (j) j = lps[j - 1];
        else i++;
    }
    return res;
}

int main() {
    auto r = findAll("abracadabra", "abr");
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [0, 7]
    return 0;
}
