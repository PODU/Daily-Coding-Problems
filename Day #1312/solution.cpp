// rand7 from rand5 via rejection sampling: combine two rand5 into uniform
// 1..25, accept 1..21, map to 1..7. Expected O(1) calls, O(1) space.
#include <bits/stdc++.h>
using namespace std;

static mt19937 rng(42);
int rand5() { return (int)(rng() % 5) + 1; } // uniform 1..5

int rand7() {
    while (true) {
        int v = (rand5() - 1) * 5 + (rand5() - 1); // uniform 0..24
        if (v < 21) return v % 7 + 1;              // accept 0..20 -> 1..7
    }
}

int main() {
    // Demonstrate a roughly uniform distribution over 1..7.
    int counts[8] = {0};
    for (int i = 0; i < 70000; i++) counts[rand7()]++;
    for (int i = 1; i <= 7; i++) cout << i << ": " << counts[i] << "\n";
    return 0;
}
