// Day 641: Smallest positive integer not expressible as a subset sum.
// Approach: scan sorted array; if a[i] > reach+1 a gap exists, else reach += a[i].
// Time: O(N), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long smallestNonSum(const vector<long long>& a) {
    long long reach = 0; // all of [1..reach] are representable
    for (long long x : a) {
        if (x > reach + 1) break;
        reach += x;
    }
    return reach + 1;
}

int main() {
    vector<long long> a = {1, 2, 3, 10};
    cout << smallestNonSum(a) << "\n"; // 7
    return 0;
}
