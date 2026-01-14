// Kadane's max subarray sum; empty subarray (0) allowed so answer is never negative. O(N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long maxSubarraySum(const vector<long long>& a) {
    long long best = 0, cur = 0;
    for (long long x : a) {
        cur = max(0LL, cur + x);
        best = max(best, cur);
    }
    return best;
}

int main() {
    cout << maxSubarraySum({34, -50, 42, 14, -5, 86}) << "\n"; // 137
    cout << maxSubarraySum({-5, -1, -8, -9}) << "\n";          // 0
    return 0;
}
