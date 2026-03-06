// Goldbach: iterate a from 2 upward, return first (a, n-a) both prime (lexicographically smallest).
// Time: O(n*sqrt(n)) worst, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

bool isPrime(long long x) {
    if (x < 2) return false;
    for (long long i = 2; i * i <= x; ++i)
        if (x % i == 0) return false;
    return true;
}

pair<long long,long long> goldbach(long long n) {
    for (long long a = 2; a <= n / 2; ++a)
        if (isPrime(a) && isPrime(n - a)) return {a, n - a};
    return {-1, -1};
}

int main() {
    long long n = 4;
    auto p = goldbach(n);
    cout << p.first << " + " << p.second << " = " << n << "\n";
    return 0;
}
