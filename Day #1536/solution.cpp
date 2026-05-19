// Maximum contiguous subarray sum (Kadane), empty subarray allowed (min 0).
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxSubarray(const vector<int>& a) {
    long long best = 0, cur = 0;
    for (int x : a) {
        cur = max(0LL, cur + x);
        best = max(best, cur);
    }
    return best;
}

int main() {
    vector<int> a = {34, -50, 42, 14, -5, 86};
    cout << maxSubarray(a) << "\n";
    return 0;
}
