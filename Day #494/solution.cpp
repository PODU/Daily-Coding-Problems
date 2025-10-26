// Day 494: Maximum circular subarray sum (empty allowed => 0).
// max(maxKadane, total - minKadane); if all negative answer is 0. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxCircularSubarray(const vector<long long>& a) {
    long long total = 0;
    long long maxK = LLONG_MIN, curMax = 0;
    long long minK = LLONG_MAX, curMin = 0;
    for (long long x : a) {
        total += x;
        curMax = max(x, curMax + x);
        maxK = max(maxK, curMax);
        curMin = min(x, curMin + x);
        minK = min(minK, curMin);
    }
    if (maxK < 0) return 0; // all negative -> empty subarray
    return max(maxK, total - minK);
}

int main() {
    cout << maxCircularSubarray({8, -1, 3, 4}) << "\n";  // 15
    cout << maxCircularSubarray({-4, 5, 1, 0}) << "\n";  // 6
    return 0;
}
