// Day 666: Minimize difference of two subset sums. Subset-sum DP over reachable sums up to total/2,
// pick the reachable sum closest to total/2, reconstruct one subset. Time O(n*sum), Space O(n*sum).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> a = {5, 10, 15, 20, 25};
    int n = a.size(), total = accumulate(a.begin(), a.end(), 0), half = total / 2;
    vector<vector<char>> dp(n + 1, vector<char>(half + 1, 0));
    for (int i = 0; i <= n; i++) dp[i][0] = 1;
    for (int i = 1; i <= n; i++)
        for (int s = 0; s <= half; s++)
            dp[i][s] = dp[i - 1][s] || (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]);
    int best = 0;
    for (int s = half; s >= 0; s--) if (dp[n][s]) { best = s; break; }
    // reconstruct subset A (sum = best)
    vector<int> A, B; int s = best;
    for (int i = n; i >= 1; i--) {
        if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) { A.push_back(a[i - 1]); s -= a[i - 1]; }
        else B.push_back(a[i - 1]);
    }
    cout << "{";
    for (size_t i = 0; i < A.size(); i++) cout << A[i] << (i + 1 < A.size() ? ", " : "");
    cout << "} and {";
    for (size_t i = 0; i < B.size(); i++) cout << B[i] << (i + 1 < B.size() ? ", " : "");
    cout << "}, difference of " << total - 2 * best << "\n";
    return 0;
}
