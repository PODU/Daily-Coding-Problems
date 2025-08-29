// Day 190: Maximum circular subarray sum, empty subarray (sum 0) allowed.
// ans = max(0, maxKadane, total - minKadane). Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxCircularSum(const vector<int>& a) {
    long long total = 0, maxK = LLONG_MIN, curMax = 0, minK = LLONG_MAX, curMin = 0;
    for (int x : a) {
        total += x;
        curMax = max((long long)x, curMax + x); maxK = max(maxK, curMax);
        curMin = min((long long)x, curMin + x); minK = min(minK, curMin);
    }
    long long ans = max(maxK, total - minK);
    return max(0LL, ans);
}

int main() {
    cout << maxCircularSum({8, -1, 3, 4}) << "\n";
    cout << maxCircularSum({-4, 5, 1, 0}) << "\n";
    return 0;
}
