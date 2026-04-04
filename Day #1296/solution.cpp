// Day 1296: Smallest positive integer not expressible as a subset sum of a sorted array.
// Greedy: track reachable range [1..r]; if next a[i] <= r+1 extend, else answer r+1. O(N) time.
#include <bits/stdc++.h>
using namespace std;

long long smallestNonSubsetSum(const vector<long long>& a) {
    long long r = 0; // can form every value in [1..r]
    for (long long x : a) {
        if (x > r + 1) break; // gap at r+1
        r += x;
    }
    return r + 1;
}

int main() {
    vector<long long> a = {1, 2, 3, 10};
    cout << smallestNonSubsetSum(a) << endl; // 7
    return 0;
}
