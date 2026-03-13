// Recover coin denominations from change-ways array A (unbounded coin change).
// dp starts {1,0,...}; if A[i] != dp[i], i is a coin -> unbounded-knapsack update. O(N^2) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> recoverCoins(const vector<int>& A) {
    int n = A.size();
    vector<long long> dp(n, 0);
    dp[0] = 1;
    vector<int> coins;
    for (int i = 1; i < n; ++i) {
        if (A[i] != (int)dp[i]) {
            coins.push_back(i);
            for (int v = i; v < n; ++v) dp[v] += dp[v - i];
        }
    }
    return coins;
}

string formatList(const vector<int>& xs) {
    int n = xs.size();
    string s;
    for (int i = 0; i < n; ++i) {
        if (i > 0) s += ", ";
        if (i == n - 1 && n > 1) s += "and ";
        s += to_string(xs[i]);
    }
    return s;
}

int main() {
    vector<int> A = {1, 0, 1, 1, 2};
    cout << formatList(recoverCoins(A)) << "\n"; // 2, 3, and 4
    return 0;
}
