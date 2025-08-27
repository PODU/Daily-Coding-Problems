// Day 181: Minimum palindrome partitioning.
// DP: palindrome table + min-cut DP with reconstruction. Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

vector<string> minPalindromePartition(const string& s) {
    int n = s.size();
    if (n == 0) return {};
    vector<vector<char>> pal(n, vector<char>(n, 0));
    for (int i = 0; i < n; i++) pal[i][i] = 1;
    for (int L = 2; L <= n; L++)
        for (int i = 0; i + L - 1 < n; i++) {
            int j = i + L - 1;
            if (s[i] == s[j] && (L == 2 || pal[i + 1][j - 1])) pal[i][j] = 1;
        }
    vector<int> cut(n + 1, INT_MAX), prev(n + 1, -1);
    cut[0] = 0;
    for (int i = 1; i <= n; i++)
        for (int j = 0; j < i; j++)
            if (pal[j][i - 1] && cut[j] != INT_MAX && cut[j] + 1 < cut[i]) {
                cut[i] = cut[j] + 1; prev[i] = j;
            }
    vector<string> res;
    for (int i = n; i > 0; i = prev[i]) res.push_back(s.substr(prev[i], i - prev[i]));
    reverse(res.begin(), res.end());
    return res;
}

void printRes(const vector<string>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); i++) cout << "\"" << v[i] << "\"" << (i + 1 < v.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    printRes(minPalindromePartition("racecarannakayak"));
    printRes(minPalindromePartition("abc"));
    return 0;
}
