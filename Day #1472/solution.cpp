// Day 1472: Largest product of any three integers.
// Track 3 largest and 2 smallest in one pass; max of two candidate products.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxProductThree(const vector<long long>& nums) {
    long long max1 = LLONG_MIN, max2 = LLONG_MIN, max3 = LLONG_MIN;
    long long min1 = LLONG_MAX, min2 = LLONG_MAX;
    for (long long n : nums) {
        if (n > max1) { max3 = max2; max2 = max1; max1 = n; }
        else if (n > max2) { max3 = max2; max2 = n; }
        else if (n > max3) { max3 = n; }
        if (n < min1) { min2 = min1; min1 = n; }
        else if (n < min2) { min2 = n; }
    }
    return max(max1 * max2 * max3, max1 * min1 * min2);
}

int main() {
    cout << maxProductThree({-10, -10, 5, 2}) << "\n";  // 500
}
