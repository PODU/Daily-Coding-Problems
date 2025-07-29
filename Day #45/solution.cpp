// rand7 from rand5: rejection sampling over 5*(rand5-1)+rand5 in 1..25,
// reject >21, map ((v-1)%7)+1. Expected O(1) amortized. rand5 from a seeded LCG.
#include <bits/stdc++.h>
using namespace std;

static long long state = 1; // deterministic seed
int rand5() {
    state = (state * 75 + 74) % 65537;
    return (int)(state % 5) + 1; // uniform-ish 1..5 for the demo
}

int rand7() {
    while (true) {
        int v = 5 * (rand5() - 1) + rand5(); // 1..25
        if (v <= 21) return (v - 1) % 7 + 1;
    }
}

int main() {
    cout << "rand7 samples:";
    for (int i = 0; i < 20; i++) cout << " " << rand7();
    cout << "\n";

    int counts[8] = {0};
    for (int i = 0; i < 7000; i++) counts[rand7()]++;
    cout << "counts over 7000 trials:";
    for (int v = 1; v <= 7; v++) cout << " " << v << ":" << counts[v];
    cout << "\n";
    return 0;
}
