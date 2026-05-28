// Day 1580: Largest divisible subset (every pair mutually divisible).
// Sort, then LIS-style DP: dp[i] = longest chain ending at i where a[i] % a[j] == 0.
// Time: O(n^2); Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> largestDivisible(vector<int> a) {
    sort(a.begin(), a.end());
    int n = a.size();
    if (n == 0) return {};
    vector<int> dp(n, 1), prev(n, -1);
    int best = 0;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < i; j++) {
            if (a[i] % a[j] == 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; prev[i] = j; }
        }
        if (dp[i] > dp[best]) best = i;
    }
    vector<int> res;
    for (int i = best; i != -1; i = prev[i]) res.push_back(a[i]);
    reverse(res.begin(), res.end());
    return res;
}

int main() {
    auto r = largestDivisible({3, 5, 10, 20, 21});
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [5, 10, 20]
    return 0;
}
