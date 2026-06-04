// Goldbach pair: sieve up to n, scan a from 2; first a where a and n-a prime.
// Smallest a => lexicographically smallest [a,b]. Time O(n log log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

pair<int,int> goldbach(int n) {
    vector<bool> isPrime(n + 1, true);
    isPrime[0] = isPrime[1] = false;
    for (int i = 2; (long long)i * i <= n; ++i)
        if (isPrime[i])
            for (int j = i * i; j <= n; j += i) isPrime[j] = false;
    for (int a = 2; a <= n - a; ++a)
        if (isPrime[a] && isPrime[n - a]) return {a, n - a};
    return {-1, -1};
}

int main() {
    int n = 4;
    pair<int,int> r = goldbach(n);
    cout << r.first << " + " << r.second << " = " << n << "\n";
    return 0;
}
