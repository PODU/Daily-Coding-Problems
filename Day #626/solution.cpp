// Largest product of three: track top-3 max and bottom-2 min in one pass.
// Answer = max(max1*max2*max3, min1*min2*max1). O(n) time, O(1) space.
#include <iostream>
#include <vector>
#include <algorithm>
#include <climits>

long long largestProductOfThree(const std::vector<int>& a) {
    long long max1 = LLONG_MIN, max2 = LLONG_MIN, max3 = LLONG_MIN;
    long long min1 = LLONG_MAX, min2 = LLONG_MAX;
    for (int x : a) {
        if (x > max1) { max3 = max2; max2 = max1; max1 = x; }
        else if (x > max2) { max3 = max2; max2 = x; }
        else if (x > max3) { max3 = x; }
        if (x < min1) { min2 = min1; min1 = x; }
        else if (x < min2) { min2 = x; }
    }
    return std::max(max1 * max2 * max3, min1 * min2 * max1);
}

int main() {
    std::vector<int> v = {-10, -10, 5, 2};
    std::cout << largestProductOfThree(v) << "\n"; // 500
    return 0;
}
