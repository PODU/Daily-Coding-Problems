// Day 210: Collatz conjecture - verify reach to 1 and find longest sequence for n <= 1e6.
// Memoized sequence lengths (cache for values <= LIMIT). Time: ~O(LIMIT * avg steps), Space: O(LIMIT).
#include <bits/stdc++.h>
using namespace std;

const int LIMIT = 1000000;
vector<int> cache(LIMIT + 1, 0);

int collatzLen(long long start) {
    vector<long long> path;
    long long m = start;
    while (m > LIMIT || cache[m] == 0) {
        path.push_back(m);
        m = (m % 2 == 0) ? m / 2 : 3 * m + 1;
    }
    int base = cache[m];
    for (auto it = path.rbegin(); it != path.rend(); ++it) {
        base++;
        if (*it <= LIMIT) cache[*it] = base;
    }
    return base;
}

int main() {
    cache[1] = 1;
    cout << "Collatz length of 27: " << collatzLen(27) << endl; // 112
    int bestN = 1, bestLen = 1;
    for (int n = 1; n <= LIMIT; ++n) {
        int l = collatzLen(n);
        if (l > bestLen) { bestLen = l; bestN = n; }
    }
    cout << "Longest sequence for n <= 1000000: n=" << bestN
         << " (length " << bestLen << ")" << endl; // n=837799 (length 525)
    return 0;
}
