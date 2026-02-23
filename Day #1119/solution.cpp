// Day 1119 - Split array into k contiguous partitions minimizing the max sum
// Binary search the answer; greedily count partitions. Time: O(n log(sum)).
#include <bits/stdc++.h>
using namespace std;

int partitionsNeeded(const vector<int>& N, long long limit) {
    int count = 1; long long cur = 0;
    for (int x : N) {
        if (cur + x > limit) { count++; cur = x; }
        else cur += x;
    }
    return count;
}

long long splitMinMax(const vector<int>& N, int k) {
    long long lo = *max_element(N.begin(), N.end());
    long long hi = accumulate(N.begin(), N.end(), 0LL);
    while (lo < hi) {
        long long mid = (lo + hi) / 2;
        if (partitionsNeeded(N, mid) <= k) hi = mid;
        else lo = mid + 1;
    }
    return lo;
}

int main() {
    vector<int> N = {5, 1, 2, 7, 3, 4};
    int k = 3;
    cout << splitMinMax(N, k) << endl; // 8
    return 0;
}
