// Largest product of three: one pass tracking 3 largest + 2 smallest.
// answer = max(max1*max2*max3, max1*min1*min2). Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long largestProduct(const vector<long long>& nums) {
    long long max1 = LLONG_MIN, max2 = LLONG_MIN, max3 = LLONG_MIN;
    long long min1 = LLONG_MAX, min2 = LLONG_MAX;
    for (long long x : nums) {
        if (x > max1) { max3 = max2; max2 = max1; max1 = x; }
        else if (x > max2) { max3 = max2; max2 = x; }
        else if (x > max3) { max3 = x; }
        if (x < min1) { min2 = min1; min1 = x; }
        else if (x < min2) { min2 = x; }
    }
    return max(max1 * max2 * max3, max1 * min1 * min2);
}

int main() {
    vector<long long> nums = {-10, -10, 5, 2};
    cout << largestProduct(nums) << "\n";
    return 0;
}
