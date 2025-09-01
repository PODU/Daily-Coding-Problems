// Day 198: Largest divisible subset.
// Sort, then DP: dp[i] = longest chain ending at i (a[j] | a[i]); track parent for reconstruction.
// Time: O(n^2), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> largestDivisibleSubset(vector<int> a) {
    int n = a.size();
    if (n == 0) return {};
    sort(a.begin(), a.end());
    vector<int> dp(n, 1), parent(n, -1);
    int best = 0;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < i; ++j)
            if (a[i] % a[j] == 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; parent[i] = j; }
        if (dp[i] > dp[best]) best = i;
    }
    vector<int> res;
    for (int i = best; i != -1; i = parent[i]) res.push_back(a[i]);
    reverse(res.begin(), res.end());
    return res;
}

void show(vector<int> a) {
    vector<int> r = largestDivisibleSubset(a);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl;
}

int main() {
    show({3, 5, 10, 20, 21}); // [5, 10, 20]
    show({1, 3, 6, 24});      // [1, 3, 6, 24]
    return 0;
}
