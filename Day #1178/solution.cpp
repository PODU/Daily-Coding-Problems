// Day 1178: Collatz sequences. Verify each reaches 1 and find the longest start <= 1e6.
// Memoized chain lengths (each value cached once). Time ~O(LIMIT), Space O(LIMIT).
#include <bits/stdc++.h>
using namespace std;

const int LIMIT = 1000000;
vector<int> memo;

int clen(long long n) {
    if (n == 1) return 1;
    if (n <= LIMIT && memo[n]) return memo[n];
    long long nxt = (n % 2 == 0) ? n / 2 : 3 * n + 1;
    int l = 1 + clen(nxt);
    if (n <= LIMIT) memo[n] = l;
    return l;
}

int main() {
    memo.assign(LIMIT + 1, 0);
    string seq; long long n = 6;
    while (true) { seq += to_string(n); if (n == 1) break; seq += " -> "; n = (n % 2 == 0) ? n / 2 : 3 * n + 1; }
    cout << seq << "\n";
    int bestN = 1, bestLen = 1;
    for (int i = 2; i <= LIMIT; i++) { int l = clen(i); if (l > bestLen) { bestLen = l; bestN = i; } }
    cout << "All sequences up to " << LIMIT << " reach 1: true\n";
    cout << "Longest sequence for n <= " << LIMIT << ": n = " << bestN << " (length " << bestLen << ")\n";
    return 0;
}
