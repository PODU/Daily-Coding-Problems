// Split array into k contiguous partitions minimizing the max partition sum.
// Binary search on the answer + greedy feasibility. Time: O(n log(sum)). Space: O(1).
#include <bits/stdc++.h>
using namespace std;

bool feasible(const vector<int>& a, int k, long long cap) {
    long long cur = 0; int parts = 1;
    for (int x : a) {
        if (x > cap) return false;
        if (cur + x > cap) { parts++; cur = x; }
        else cur += x;
    }
    return parts <= k;
}

long long splitArray(const vector<int>& a, int k) {
    long long lo = 0, hi = 0;
    for (int x : a) { lo = max(lo, (long long)x); hi += x; }
    while (lo < hi) {
        long long mid = lo + (hi - lo) / 2;
        if (feasible(a, k, mid)) hi = mid; else lo = mid + 1;
    }
    return lo;
}

int main() {
    vector<int> N = {5, 1, 2, 7, 3, 4};
    cout << splitArray(N, 3) << "\n"; // 8
    return 0;
}
