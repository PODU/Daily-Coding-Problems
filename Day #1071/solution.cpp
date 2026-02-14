// Subset sum DP: dp[i][s] = reachable using first i items; reconstruct path. O(n*k) time/space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> items = {12, 1, 61, 5, 9, 2};
    int k = 24;
    int n = items.size();

    auto solve = [&](int target) -> vector<int> {
        vector<vector<bool>> dp(n+1, vector<bool>(target+1, false));
        dp[0][0] = true;
        for (int i = 1; i <= n; i++) {
            for (int s = 0; s <= target; s++) {
                dp[i][s] = dp[i-1][s];
                if (!dp[i][s] && s >= items[i-1])
                    dp[i][s] = dp[i-1][s - items[i-1]];
            }
        }
        if (!dp[n][target]) return {};
        vector<int> result;
        int s = target;
        for (int i = n; i >= 1; i--) {
            if (!dp[i-1][s]) {
                result.push_back(items[i-1]);
                s -= items[i-1];
            }
        }
        reverse(result.begin(), result.end());
        return result;
    };

    auto res = solve(k);
    cout << "Subset: [";
    for (int i = 0; i < (int)res.size(); i++) {
        if (i) cout << ", ";
        cout << res[i];
    }
    cout << "]\n";
    int sum = 0; for (int x : res) sum += x;
    cout << "Sum: " << sum << "\n";

    auto res2 = solve(1000);
    if (res2.empty()) cout << "null\n";

    return 0;
}
