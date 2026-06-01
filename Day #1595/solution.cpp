// Approach: DP min palindrome partition. isPal[i][j] table O(n^2), cut[i]=min cuts for prefix,
// then reconstruct one optimal partition. Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

vector<string> minPalPartition(const string& s) {
    int n = s.size();
    if (n == 0) return {};
    vector<vector<bool>> pal(n, vector<bool>(n, false));
    for (int i = 0; i < n; i++) pal[i][i] = true;
    for (int len = 2; len <= n; len++)
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            if (s[i] == s[j] && (len == 2 || pal[i+1][j-1])) pal[i][j] = true;
        }
    vector<int> cut(n, 0);
    vector<int> start(n, 0); // start index of last palindrome ending at i
    for (int i = 0; i < n; i++) {
        if (pal[0][i]) { cut[i] = 0; start[i] = 0; continue; }
        int best = INT_MAX, bj = 0;
        for (int j = 1; j <= i; j++)
            if (pal[j][i] && cut[j-1] + 1 < best) { best = cut[j-1] + 1; bj = j; }
        cut[i] = best; start[i] = bj;
    }
    vector<string> res;
    int i = n - 1;
    while (i >= 0) { int j = start[i]; res.push_back(s.substr(j, i - j + 1)); i = j - 1; }
    reverse(res.begin(), res.end());
    return res;
}

void printList(const vector<string>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); i++) { cout << "\"" << v[i] << "\""; if (i + 1 < v.size()) cout << ", "; }
    cout << "]\n";
}

int main() {
    printList(minPalPartition("racecarannakayak"));
    printList(minPalPartition("abc"));
    return 0;
}
