// Iterate integers, sum digits, count until the n-th whose digit sum is 10.
// Time O(answer * digits), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int digitSum(long long x) {
    int s = 0;
    while (x) { s += x % 10; x /= 10; }
    return s;
}

long long nthPerfect(int n) {
    int count = 0;
    long long num = 0;
    while (count < n) {
        num++;
        if (digitSum(num) == 10) count++;
    }
    return num;
}

int main() {
    cout << nthPerfect(1) << endl;
    return 0;
}
