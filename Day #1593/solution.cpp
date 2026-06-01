// rand5 from rand7 via rejection sampling: draw rand7, accept if <=5 else retry.
// Expected O(1) calls (acceptance prob 5/7). Output uniform on 1..5.
#include <bits/stdc++.h>
using namespace std;

mt19937 rng(12345); // deterministic seed

// uniform 1..7 using language RNG
int rand7() {
    uniform_int_distribution<int> d(1, 7);
    return d(rng);
}

// uniform 1..5 via rejection sampling
int rand5() {
    while (true) {
        int v = rand7();
        if (v <= 5) return v;
    }
}

int main() {
    const int N = 100000;
    int counts[6] = {0};
    for (int i = 0; i < N; ++i) counts[rand5()]++;

    cout << "Distribution over " << N << " samples:\n";
    for (int v = 1; v <= 5; ++v)
        cout << "  " << v << ": " << counts[v] << "\n";

    cout << "First 10 samples:";
    mt19937 rng2(999);
    swap(rng, rng2);
    for (int i = 0; i < 10; ++i) cout << " " << rand5();
    cout << "\n";
    return 0;
}
