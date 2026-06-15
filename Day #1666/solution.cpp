// Day 1666: GCD of n numbers.
// Approach: fold Euclid's algorithm across the list. Time O(n*log(max)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long gcd2(long long a, long long b) {
    while (b) { a %= b; swap(a, b); }
    return a;
}

long long gcdList(const vector<long long>& nums) {
    long long g = 0;
    for (long long x : nums) g = gcd2(g, llabs(x));
    return g;
}

int main() {
    vector<long long> nums = {42, 56, 14};
    cout << gcdList(nums) << "\n"; // 14
    return 0;
}
