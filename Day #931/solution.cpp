// Day 931: GCD of n numbers by folding pairwise gcd.
// Time: O(n * log(maxVal)), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long gcdList(const vector<long long>& nums) {
    long long g = 0;
    for (long long x : nums) g = std::__gcd(g, x);
    return g;
}

int main() {
    vector<long long> nums = {42, 56, 14};
    cout << gcdList(nums) << "\n"; // 14
    return 0;
}
