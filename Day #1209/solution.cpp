// Day 1209: Largest divisible subset.
// Sort, dp[i]=longest chain ending at i with parent pointers. Time O(n^2), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> largestDivisibleSubset(vector<int> a) {
    sort(a.begin(), a.end());
    int n = a.size();
    if (n == 0) return {};
    vector<int> dp(n, 1), par(n, -1);
    int best = 0;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < i; j++)
            if (a[i] % a[j] == 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; par[i] = j; }
        if (dp[i] > dp[best]) best = i;
    }
    vector<int> res;
    for (int i = best; i != -1; i = par[i]) res.push_back(a[i]);
    reverse(res.begin(), res.end());
    return res;
}

int main() {
    auto r = largestDivisibleSubset({3, 5, 10, 20, 21});
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) { if (i) cout << ", "; cout << r[i]; }
    cout << "]\n"; // [5, 10, 20]
    return 0;
}
