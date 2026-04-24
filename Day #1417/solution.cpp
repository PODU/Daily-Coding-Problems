// Day 1417: count tilings of a 2xN board with dominoes and L-trominoes.
// Approach: DP recurrence f(n) = 2*f(n-1) + f(n-3), f(0)=1,f(1)=1,f(2)=2. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long countTilings(int n) {
    if (n == 0) return 1;
    if (n == 1) return 1;
    if (n == 2) return 2;
    long long a = 1, b = 1, c = 2; // f(0), f(1), f(2)
    for (int i = 3; i <= n; ++i) {
        long long cur = 2 * c + a;
        a = b; b = c; c = cur;
    }
    return c;
}

int main() {
    cout << countTilings(4) << "\n"; // 11
    return 0;
}
