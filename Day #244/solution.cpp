// Sieve of Eratosthenes: primes < N in O(N log log N) time, O(N) space.
// Plus an indefinite incremental prime generator (stateful struct) using trial division by known primes.
#include <bits/stdc++.h>
using namespace std;

vector<int> sieve(int n) {
    vector<bool> is_comp(max(n, 0), false);
    vector<int> primes;
    for (int i = 2; i < n; ++i) {
        if (!is_comp[i]) {
            primes.push_back(i);
            for (long long j = (long long)i * i; j < n; j += i) is_comp[j] = true;
        }
    }
    return primes;
}

// Indefinite generator: yields primes one at a time via trial division by previously found primes.
struct PrimeGen {
    vector<long long> primes;
    long long cand = 1;
    long long next() {
        while (true) {
            ++cand;
            bool prime = true;
            for (long long p : primes) {
                if (p * p > cand) break;
                if (cand % p == 0) { prime = false; break; }
            }
            if (prime) { primes.push_back(cand); return cand; }
        }
    }
};

int main() {
    vector<int> p = sieve(100);
    for (size_t i = 0; i < p.size(); ++i) cout << (i ? " " : "") << p[i];
    cout << "\n";

    PrimeGen g;
    for (int i = 0; i < 10; ++i) cout << (i ? " " : "") << g.next();
    cout << "\n";
    return 0;
}
