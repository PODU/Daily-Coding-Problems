// Day 420: n-th positive integer whose digits sum to exactly 10.
// Iterate integers, count those with digit sum 10. Time: O(answer), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int digitSum(long long x) {
    int s = 0;
    while (x > 0) { s += x % 10; x /= 10; }
    return s;
}

long long nthPerfect(int n) {
    int count = 0;
    long long x = 0;
    while (count < n) {
        ++x;
        if (digitSum(x) == 10) ++count;
    }
    return x;
}

int main() {
    cout << nthPerfect(1) << endl; // 19
    cout << nthPerfect(2) << endl; // 28
    return 0;
}
