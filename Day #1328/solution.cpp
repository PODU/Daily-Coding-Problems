// Day 1328: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i]=min pieces for prefix i, with parent pointers to rebuild. O(n^2) time/space.
#include <bits/stdc++.h>
using namespace std;

vector<string> minPalindromePartition(const string& s) {
    int n = s.size();
    if (n == 0) return {};
    vector<vector<bool>> pal(n, vector<bool>(n, false));
    for (int i = n - 1; i >= 0; i--)
        for (int j = i; j < n; j++)
            pal[i][j] = (s[i] == s[j]) && (j - i < 2 || pal[i + 1][j - 1]);

    vector<int> dp(n + 1, INT_MAX), prev(n + 1, -1);
    dp[0] = 0;
    for (int i = 1; i <= n; i++)
        for (int j = 0; j < i; j++)
            if (pal[j][i - 1] && dp[j] != INT_MAX && dp[j] + 1 < dp[i]) {
                dp[i] = dp[j] + 1; prev[i] = j;
            }

    vector<string> res;
    for (int i = n; i > 0; i = prev[i]) res.push_back(s.substr(prev[i], i - prev[i]));
    reverse(res.begin(), res.end());
    return res;
}

void print(const vector<string>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); i++) cout << "\"" << v[i] << "\"" << (i + 1 < v.size() ? ", " : "");
    cout << "]" << endl;
}

int main() {
    print(minPalindromePartition("racecarannakayak")); // ["racecar", "anna", "kayak"]
    print(minPalindromePartition("abc"));               // ["a", "b", "c"]
    return 0;
}
