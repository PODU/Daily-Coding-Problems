// Day 943: Count tilings of a 2xN board with 2x1 dominoes and L-trominoes.
// DP recurrence f(n) = 2*f(n-1) + f(n-3), f(0)=1,f(1)=1,f(2)=2. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long countTilings(int n) {
    if (n == 0) return 1;
    if (n == 1) return 1;
    if (n == 2) return 2;
    long long a = 1, b = 1, c = 2; // f(0), f(1), f(2)
    for (int i = 3; i <= n; ++i) {
        long long f = 2 * c + a; // f(i) = 2*f(i-1) + f(i-3)
        a = b; b = c; c = f;
    }
    return c;
}

int main() {
    cout << countTilings(4) << "\n"; // 11
    return 0;
}
