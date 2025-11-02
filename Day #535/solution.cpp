// Egg drop (min worst-case trials): find smallest m such that with N eggs we can
// cover k floors. f(m, N) = sum_{i=1..N} C(m,i); smallest m with f(m,N) >= k.
// Time: O(m * N) where m is the answer; Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int eggDrop(long long N, long long k) {
    int m = 0;
    long long covered = 0;
    while (covered < k) {
        m++;
        // floors coverable with m moves and N eggs = sum_{i=1..N} C(m,i)
        long long sum = 0, term = 1; // term = C(m, i)
        for (int i = 1; i <= N; i++) {
            term = term * (m - i + 1) / i; // C(m,i)
            sum += term;
            if (sum >= k) break;
        }
        covered = sum;
    }
    return m;
}

int main() {
    cout << eggDrop(1, 5) << endl; // expected 5
    return 0;
}
