// Day 1002: Smallest positive integer not expressible as a subset sum (sorted array).
// If the next element x <= res (smallest unreachable, init 1) extend to res+x,
// else res is the answer. O(N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long smallestNonSubsetSum(const vector<long long>& nums) {
    long long res = 1;
    for (long long x : nums) {
        if (x > res) break;
        res += x;
    }
    return res;
}

int main() {
    cout << smallestNonSubsetSum({1, 2, 3, 10}) << "\n"; // 7
    return 0;
}
