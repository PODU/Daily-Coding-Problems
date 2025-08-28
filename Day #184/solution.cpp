// Day 184: GCD of n numbers via iterated Euclidean algorithm.
// Time O(n * log(max)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long gcd2(long long a, long long b) { while (b) { a %= b; swap(a, b); } return a; }

long long gcdAll(const vector<long long>& nums) {
    long long g = 0;
    for (long long x : nums) g = gcd2(g, x);
    return g;
}

int main() {
    vector<long long> nums = {42, 56, 14};
    cout << gcdAll(nums) << "\n";
    return 0;
}
