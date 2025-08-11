// Day 101: Goldbach. Sieve primes up to n, then the smallest prime a with n-a
// prime gives the lexicographically smallest pair. O(n log log n).
#include <bits/stdc++.h>
using namespace std;

pair<int, int> goldbach(int n) {
    vector<bool> sieve(n + 1, true);
    sieve[0] = sieve[1] = false;
    for (int i = 2; (long long)i * i <= n; i++)
        if (sieve[i])
            for (int j = i * i; j <= n; j += i) sieve[j] = false;
    for (int a = 2; a <= n / 2; a++)
        if (sieve[a] && sieve[n - a]) return {a, n - a};
    return {-1, -1};
}

int main() {
    auto [a, b] = goldbach(4);
    cout << a << " + " << b << " = " << (a + b) << "\n"; // 2 + 2 = 4
    return 0;
}
