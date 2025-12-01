// Sieve of Eratosthenes for primes < N (O(N log log N)); plus an incremental
// generator using a map of next composite multiples. Space: O(N) sieve.
#include <iostream>
#include <vector>
#include <map>
using namespace std;

vector<int> sieve(int n) {
    vector<bool> isComp(n < 0 ? 0 : n, false);
    vector<int> primes;
    for (int i = 2; i < n; i++) {
        if (!isComp[i]) {
            primes.push_back(i);
            for (long long j = (long long)i * i; j < n; j += i) isComp[j] = true;
        }
    }
    return primes;
}

// Incremental generator: yields primes one-by-one via a map of next composites.
class PrimeGen {
    map<long long, long long> composites; // composite -> its prime factor
    long long current = 1;
public:
    long long next() {
        while (true) {
            current++;
            auto it = composites.find(current);
            if (it == composites.end()) {
                composites[current * current] = current; // prime found
                return current;
            } else {
                long long prime = it->second;
                composites.erase(it);
                long long x = current + prime;
                while (composites.count(x)) x += prime;
                composites[x] = prime;
            }
        }
    }
};

int main() {
    vector<int> primes = sieve(100);
    cout << "[";
    for (size_t i = 0; i < primes.size(); i++)
        cout << primes[i] << (i + 1 < primes.size() ? ", " : "");
    cout << "]\n";

    PrimeGen gen;
    cout << "[";
    for (int i = 0; i < 10; i++)
        cout << gen.next() << (i + 1 < 10 ? ", " : "");
    cout << "]\n";
    return 0;
}
