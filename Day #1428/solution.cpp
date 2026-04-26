// Day 1428: Partition array into two subsets minimizing sum difference.
// Approach: subset-sum DP over half the total; reconstruct one subset.
// Time: O(n * sum), Space: O(n * sum).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> a = {5, 10, 15, 20, 25};
    int n = a.size();
    int total = 0;
    for (int x : a) total += x;
    int half = total / 2;

    // dp[i][s] = can we pick from first i items to reach sum s
    vector<vector<char>> dp(n + 1, vector<char>(half + 1, 0));
    for (int i = 0; i <= n; ++i) dp[i][0] = 1;
    for (int i = 1; i <= n; ++i)
        for (int s = 0; s <= half; ++s) {
            dp[i][s] = dp[i - 1][s];
            if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) dp[i][s] = 1;
        }

    int best = 0;
    for (int s = half; s >= 0; --s)
        if (dp[n][s]) { best = s; break; }

    // reconstruct subset summing to best
    vector<int> subset;
    int s = best;
    for (int i = n; i >= 1; --i) {
        if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) {
            subset.push_back(a[i - 1]);
            s -= a[i - 1];
        }
    }

    int diff = total - 2 * best;
    cout << "Difference: " << diff << "\nSubset: {";
    for (size_t i = 0; i < subset.size(); ++i)
        cout << subset[i] << (i + 1 < subset.size() ? ", " : "");
    cout << "}\n"; // Difference: 5
    return 0;
}
