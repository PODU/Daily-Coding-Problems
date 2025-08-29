// Day 186: Minimum subset-sum difference (partition problem).
// Subset-sum DP over total, pick best <= total/2, reconstruct. Time O(n*S), Space O(n*S).
#include <bits/stdc++.h>
using namespace std;

void solve(const vector<int>& a) {
    int n = a.size(), tot = accumulate(a.begin(), a.end(), 0);
    vector<vector<char>> dp(n + 1, vector<char>(tot + 1, 0));
    dp[0][0] = 1;
    for (int i = 1; i <= n; i++)
        for (int s = 0; s <= tot; s++)
            dp[i][s] = dp[i - 1][s] || (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]);
    int best = 0;
    for (int s = tot / 2; s >= 0; s--) if (dp[n][s]) { best = s; break; }
    vector<int> sub, other;
    int s = best;
    for (int i = n; i >= 1; i--) {
        if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) { sub.push_back(a[i - 1]); s -= a[i - 1]; }
        else other.push_back(a[i - 1]);
    }
    reverse(sub.begin(), sub.end()); reverse(other.begin(), other.end());
    auto pr = [](const vector<int>& v) {
        cout << "{";
        for (size_t i = 0; i < v.size(); i++) cout << v[i] << (i + 1 < v.size() ? ", " : "");
        cout << "}";
    };
    pr(sub); cout << " and "; pr(other); cout << "\n";
}

int main() {
    solve({5, 10, 15, 20, 25});
    return 0;
}
