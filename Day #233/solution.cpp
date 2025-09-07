// Fibonacci in O(1) space: iterate keeping only the last two values.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

long long fib(int n) {
    if (n < 2) return n;
    long long a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        long long c = a + b;
        a = b;
        b = c;
    }
    return b;
}

int main() {
    cout << fib(10) << "\n"; // 55
}
