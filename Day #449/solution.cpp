// Day 449: Goldbach pair. Scan a from 2 upward; the first a where a and n-a are
// both prime gives the lexicographically smallest pair. O(n*sqrt(n)) worst case.
#include <bits/stdc++.h>
using namespace std;

bool isPrime(int x) {
    if (x < 2) return false;
    for (int i = 2; (long long)i * i <= x; ++i) if (x % i == 0) return false;
    return true;
}

pair<int,int> goldbach(int n) {
    for (int a = 2; a <= n / 2; ++a)
        if (isPrime(a) && isPrime(n - a)) return {a, n - a};
    return {-1, -1};
}

int main() {
    int n = 4;
    auto p = goldbach(n);
    cout << p.first << " + " << p.second << " = " << n << "\n"; // 2 + 2 = 4
    return 0;
}
