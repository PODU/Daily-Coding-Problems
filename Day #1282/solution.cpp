// Day 1282: n-th positive integer whose digits sum to 10.
// Such numbers are ~ every 9th integer; iterate counting matches. Time O(answer/9), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int digitSum(long long x) {
    int s = 0;
    while (x) { s += x % 10; x /= 10; }
    return s;
}

long long nthPerfect(int n) {
    long long x = 0;
    int count = 0;
    while (count < n) {
        ++x;
        if (digitSum(x) == 10) ++count;
    }
    return x;
}

int main() {
    cout << nthPerfect(1) << "\n"; // 19
    cout << nthPerfect(2) << "\n"; // 28
    return 0;
}
