// Day 1175: Maximum subarray sum in a circular array (empty allowed -> 0).
// Answer = max(0, kadaneMax, total - kadaneMin); total-min covers the wrap case.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxCircularSubarray(const vector<int>& a) {
    long long total = 0, curMax = 0, bestMax = 0, curMin = 0, bestMin = 0;
    for (int x : a) {
        total += x;
        curMax = max((long long)x, curMax + x); bestMax = max(bestMax, curMax);
        curMin = min((long long)x, curMin + x); bestMin = min(bestMin, curMin);
    }
    long long wrap = total - bestMin;            // best non-wrap of the complement
    return max(0LL, max(bestMax, wrap));
}

int main() {
    cout << maxCircularSubarray({8, -1, 3, 4}) << "\n"; // 15
    cout << maxCircularSubarray({-4, 5, 1, 0}) << "\n"; // 6
    return 0;
}
