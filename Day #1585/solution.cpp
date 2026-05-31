// Day 1585: Sieve of Eratosthenes + incremental infinite prime generator.
// Sieve marks composites up to N. Generator uses a "wheel" map of next composite per prime.
// Time: O(N log log N) sieve; Space: O(N). Generator: ~O(1) amortized per prime.
#include <bits/stdc++.h>
using namespace std;

vector<int> sieve(int n) {
    vector<bool> comp(n, false);
    vector<int> primes;
    for (int i = 2; i < n; i++) {
        if (!comp[i]) {
            primes.push_back(i);
            for (long long j = (long long)i * i; j < n; j += i) comp[j] = true;
        }
    }
    return primes;
}

// Infinite generator via incremental sieve (returns first `count` primes).
vector<int> firstPrimes(int count) {
    vector<int> primes;
    unordered_map<long long, vector<long long>> composites; // num -> list of primes that hit it
    for (long long n = 2; (int)primes.size() < count; n++) {
        auto it = composites.find(n);
        if (it == composites.end()) {                       // n is prime
            primes.push_back((int)n);
            composites[n * n].push_back(n);
        } else {
            for (long long p : it->second) composites[n + p].push_back(p);
            composites.erase(it);
        }
    }
    return primes;
}

int main() {
    auto p = sieve(30);
    cout << "Primes < 30: ";
    for (size_t i = 0; i < p.size(); i++) cout << p[i] << (i + 1 < p.size() ? " " : "");
    cout << "\n";
    auto g = firstPrimes(10);
    cout << "First 10 primes: ";
    for (size_t i = 0; i < g.size(); i++) cout << g[i] << (i + 1 < g.size() ? " " : "");
    cout << "\n";
    return 0;
}
