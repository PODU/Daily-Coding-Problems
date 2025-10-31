// Kadane's algorithm: max contiguous subarray sum, empty subarray allowed (>=0).
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxSubarraySum(const vector<long long>& a) {
    long long best = 0, cur = 0; // empty subarray allowed -> min 0
    for (long long x : a) {
        cur = max(0LL, cur + x);
        best = max(best, cur);
    }
    return best;
}

int main() {
    cout << maxSubarraySum({34, -50, 42, 14, -5, 86}) << "\n";
    cout << maxSubarraySum({-5, -1, -8, -9}) << "\n";
    return 0;
}
