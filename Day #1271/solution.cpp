// Day 1271: Implement rand5() from rand7() with uniform probability.
// Rejection sampling: redraw rand7 until result <= 5. Expected O(7/5) calls per sample.
#include <bits/stdc++.h>
using namespace std;

mt19937 rng(random_device{}());

int rand7() { return uniform_int_distribution<int>(1, 7)(rng); }

int rand5() {
    int v;
    do { v = rand7(); } while (v > 5); // values 6,7 rejected -> uniform 1..5
    return v;
}

int main() {
    array<int,6> count{}; // index 1..5
    int trials = 100000;
    for (int i = 0; i < trials; ++i) count[rand5()]++;
    cout << "Distribution over " << trials << " samples (expect ~" << trials / 5 << " each):\n";
    for (int v = 1; v <= 5; ++v) cout << v << ": " << count[v] << "\n";
    return 0;
}
