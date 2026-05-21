// Largest product of any three integers.
// Track 3 largest and 2 smallest in one pass; answer = max(top3, min2*top1).
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long largestProductOfThree(const vector<int>& a) {
    long long max1 = LLONG_MIN, max2 = LLONG_MIN, max3 = LLONG_MIN;
    long long min1 = LLONG_MAX, min2 = LLONG_MAX;
    for (int x : a) {
        if (x >= max1) { max3 = max2; max2 = max1; max1 = x; }
        else if (x >= max2) { max3 = max2; max2 = x; }
        else if (x > max3) { max3 = x; }
        if (x <= min1) { min2 = min1; min1 = x; }
        else if (x < min2) { min2 = x; }
    }
    return max(max1 * max2 * max3, min1 * min2 * max1);
}

int main() {
    vector<int> a = {-10, -10, 5, 2};
    cout << largestProductOfThree(a) << "\n";
    return 0;
}
