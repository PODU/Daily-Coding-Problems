// Iterative rolling Fibonacci: two variables, O(n) time, O(1) space.
// fib(0)=0, fib(1)=1.
#include <bits/stdc++.h>
using namespace std;

long long fib(int n) {
    if (n < 2) return n;
    long long a = 0, b = 1;
    for (int i = 2; i <= n; ++i) {
        long long c = a + b;
        a = b;
        b = c;
    }
    return b;
}

int main() {
    cout << fib(10) << "\n";
    return 0;
}
