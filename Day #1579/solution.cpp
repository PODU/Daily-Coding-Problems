// Day 1579: Maximum circular subarray sum (empty allowed -> 0).
// ans = max(Kadane-with-empty, total - minSubarray). Time: O(n); Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxCircular(const vector<int>& a) {
    long long total = 0, maxEnd = 0, maxSum = 0;     // empty allowed -> baseline 0
    long long minEnd = 0, minSum = LLONG_MAX;        // min non-empty subarray
    for (int x : a) {
        total += x;
        maxEnd = max((long long)x, maxEnd + x);
        maxSum = max(maxSum, maxEnd);
        minEnd = min((long long)x, minEnd + x);
        minSum = min(minSum, minEnd);
    }
    long long wrap = total - minSum;                 // total minus smallest middle part
    return max(maxSum, wrap);
}

int main() {
    cout << maxCircular({8, -1, 3, 4}) << "\n"; // 15
    cout << maxCircular({-4, 5, 1, 0}) << "\n"; // 6
    return 0;
}
