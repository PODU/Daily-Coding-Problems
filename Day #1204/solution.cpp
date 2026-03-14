// Day 1204: GCD of n numbers.
// Fold Euclidean gcd across the list. Time O(n log max), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long gcdAll(const vector<long long>& a) {
    long long g = 0;
    for (long long x : a) g = __gcd(g, x);
    return g;
}

int main() {
    cout << gcdAll({42, 56, 14}) << "\n"; // 14
    return 0;
}
