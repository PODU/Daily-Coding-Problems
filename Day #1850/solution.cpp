// Day 1850: rand7() from rand5() via rejection sampling on the 1..25 grid.
// Expected O(1) amortized calls (acceptance 21/25); uniform over 1..7.
#include <bits/stdc++.h>
using namespace std;

mt19937 rng(12345);
int rand5() { return uniform_int_distribution<int>(1, 5)(rng); }

int rand7() {
    while (true) {
        int v = 5 * (rand5() - 1) + rand5();  // uniform in 1..25
        if (v <= 21) return (v - 1) % 7 + 1;  // keep 21 values -> 3 each per 1..7
    }
}

int main() {
    int counts[8] = {0};
    for (int i = 0; i < 70000; i++) counts[rand7()]++;
    cout << "Sample of 10:";
    for (int i = 0; i < 10; i++) cout << " " << rand7();
    cout << "\nHistogram over 70000 draws (each ~10000):\n";
    for (int i = 1; i <= 7; i++) cout << "  " << i << ": " << counts[i] << "\n";
}
