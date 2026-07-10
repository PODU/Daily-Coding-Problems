// fib(n): iterative two-variable rolling sum.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long fib(int n) {
    long long a = 0, b = 1;
    for (int i = 0; i < n; i++) {
        long long t = a + b;
        a = b;
        b = t;
    }
    return a;
}

int main() {
    cout << fib(10) << "\n";
    return 0;
}
