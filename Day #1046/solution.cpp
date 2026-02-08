// Shortest palindrome by inserting chars; lexicographically earliest on ties.
// DP dp[i][j]=min insertions; memoized build(i,j) reconstructs string. Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

int n;
string s;
vector<vector<int>> dp;
vector<vector<string>> memo;
vector<vector<char>> done;

string build(int i, int j) {
    if (i > j) return "";
    if (i == j) return string(1, s[i]);
    if (done[i][j]) return memo[i][j];
    string res;
    if (s[i] == s[j]) {
        res = string(1, s[i]) + build(i + 1, j - 1) + string(1, s[i]);
    } else {
        string a = string(1, s[i]) + build(i + 1, j) + string(1, s[i]);
        string b = string(1, s[j]) + build(i, j - 1) + string(1, s[j]);
        if (dp[i + 1][j] < dp[i][j - 1]) res = a;
        else if (dp[i][j - 1] < dp[i + 1][j]) res = b;
        else res = min(a, b);
    }
    done[i][j] = 1;
    memo[i][j] = res;
    return res;
}

string solve(const string& str) {
    s = str;
    n = (int)s.size();
    dp.assign(n, vector<int>(n, 0));
    memo.assign(n, vector<string>(n, ""));
    done.assign(n, vector<char>(n, 0));
    for (int len = 2; len <= n; len++) {
        for (int i = 0; i + len - 1 < n; i++) {
            int j = i + len - 1;
            if (s[i] == s[j]) dp[i][j] = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : 0;
            else dp[i][j] = 1 + min(dp[i + 1][j], dp[i][j - 1]);
        }
    }
    if (n == 0) return "";
    return build(0, n - 1);
}

int main() {
    cout << "race -> " << solve("race") << "\n";
    cout << "google -> " << solve("google") << "\n";
    return 0;
}
