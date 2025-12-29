// Sieve of Eratosthenes for primes below N; incremental generator via trial division by sqrt. O(N log log N) sieve.
#include <iostream>
#include <vector>
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

// Incremental generator: trial division of candidates against found primes up to sqrt.
vector<int> firstNPrimes(int count) {
    vector<int> primes;
    int cand = 2;
    while ((int)primes.size() < count) {
        bool isPrime = true;
        for (int p : primes) {
            if ((long long)p * p > cand) break;
            if (cand % p == 0) { isPrime = false; break; }
        }
        if (isPrime) primes.push_back(cand);
        cand++;
    }
    return primes;
}

void print(const string& label, const vector<int>& v) {
    cout << label << "[";
    for (size_t i = 0; i < v.size(); i++) {
        cout << v[i];
        if (i + 1 < v.size()) cout << ", ";
    }
    cout << "]\n";
}

int main() {
    print("Primes below 30: ", sieve(30));
    print("First 10 primes: ", firstNPrimes(10));
    return 0;
}
