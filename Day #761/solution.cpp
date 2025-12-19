// Day 761: rand5() from rand7() via rejection sampling.
// Accept values 1..5, reject 6..7 and retry. Uniform; expected O(1) calls (7/5).
#include <bits/stdc++.h>
using namespace std;

struct LCG { unsigned long long s; unsigned int next(){ s=(s*1103515245ULL+12345ULL)&0x7fffffffULL; return (unsigned)s; } } rng{1};

int rand7() { return (int)(rng.next() % 7) + 1; }   // uniform 1..7

int rand5() {
    int x;
    do { x = rand7(); } while (x > 5);              // reject 6,7
    return x;
}

int main() {
    long long N = 100000, cnt[6] = {0};
    for (long long i = 0; i < N; ++i) cnt[rand5()]++;
    cout << "counts over " << N << " samples:\n";
    for (int v = 1; v <= 5; ++v) cout << v << ": " << cnt[v] << "\n";
    return 0;
}
