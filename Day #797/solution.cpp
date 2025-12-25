// Day 797: Goldbach - two primes summing to even n, lexicographically smallest.
// Scan a from 2 upward; first prime a with prime (n-a) gives smallest pair.
// Time O(n * sqrt(n)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isPrime(int x) {
    if (x < 2) return false;
    for (int d = 2; (long long)d * d <= x; d++)
        if (x % d == 0) return false;
    return true;
}

pair<int, int> goldbach(int n) {
    for (int a = 2; a <= n / 2; a++)
        if (isPrime(a) && isPrime(n - a)) return {a, n - a};
    return {-1, -1};
}

int main() {
    int n = 4;
    auto [a, b] = goldbach(n);
    cout << a << " + " << b << " = " << n << "\n"; // 2 + 2 = 4
    return 0;
}
