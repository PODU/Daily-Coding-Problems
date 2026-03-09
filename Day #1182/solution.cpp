// Day 1182: Split N into k contiguous partitions minimizing the maximum sum.
// Binary search the answer in [max element, total]; greedy feasibility check.
// Time O(N log(sum)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool feasible(const vector<int>& a, int k, long long cap) {
    long long cur = 0; int parts = 1;
    for (int x : a) {
        if (cur + x > cap) { parts++; cur = x; if (parts > k) return false; }
        else cur += x;
    }
    return true;
}

long long splitArray(const vector<int>& a, int k) {
    long long lo = *max_element(a.begin(), a.end());
    long long hi = accumulate(a.begin(), a.end(), 0LL);
    while (lo < hi) {
        long long mid = lo + (hi - lo) / 2;
        if (feasible(a, k, mid)) hi = mid;
        else lo = mid + 1;
    }
    return lo;
}

int main() {
    cout << splitArray({5, 1, 2, 7, 3, 4}, 3) << "\n"; // 8
    return 0;
}
