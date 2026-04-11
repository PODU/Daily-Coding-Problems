// Sieve of Eratosthenes: mark multiples of each prime from p*p. O(N log log N) time, O(N) space.
// Bonus: an incremental prime generator yielding primes indefinitely.
#include <bits/stdc++.h>
using namespace std;

vector<int> sieve(int n) {
    vector<char> is_prime(max(n, 2), true);
    if (n > 0) is_prime[0] = false;
    if (n > 1) is_prime[1] = false;
    for (int p = 2; (long long)p * p < n; ++p)
        if (is_prime[p])
            for (int m = p * p; m < n; m += p) is_prime[m] = false;
    vector<int> primes;
    for (int i = 2; i < n; ++i) if (is_prime[i]) primes.push_back(i);
    return primes;
}

// Incremental sieve generator: produces primes one at a time, indefinitely.
struct PrimeGenerator {
    map<long long, vector<long long>> composites;
    long long candidate = 1;
    long long next() {
        while (true) {
            ++candidate;
            auto it = composites.find(candidate);
            if (it == composites.end()) {
                composites[candidate * candidate].push_back(candidate);
                return candidate;
            } else {
                for (long long p : it->second)
                    composites[candidate + p].push_back(p);
                composites.erase(it);
            }
        }
    }
};

int main() {
    vector<int> primes = sieve(100);
    for (size_t i = 0; i < primes.size(); ++i)
        cout << primes[i] << (i + 1 < primes.size() ? " " : "");
    cout << "\n";

    PrimeGenerator gen;
    for (int i = 0; i < 10; ++i)
        cout << gen.next() << (i + 1 < 10 ? " " : "");
    cout << "\n";
    return 0;
}
