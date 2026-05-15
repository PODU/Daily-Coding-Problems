// Two dice games via Monte Carlo simulation. Expected rolls: "5 then 6"=36, "5 then 5"=42.
// Time: O(trials * rolls_per_trial). Space: O(1).
#include <bits/stdc++.h>
using namespace std;

double simulate(int first, int second, long long trials, mt19937& rng) {
    uniform_int_distribution<int> die(1, 6);
    long long total = 0;
    for (long long t = 0; t < trials; t++) {
        int prev = 0, rolls = 0;
        for (;;) {
            int r = die(rng);
            rolls++;
            if (prev == first && r == second) break;
            prev = r;
        }
        total += rolls;
    }
    return (double)total / trials;
}

int main() {
    mt19937 rng(12345);
    const long long trials = 500000;
    double e1 = simulate(5, 6, trials, rng);
    double e2 = simulate(5, 5, trials, rng);
    cout << fixed << setprecision(2);
    cout << "Game 1 (five then six) expected rolls: " << e1 << "\n";
    cout << "Game 2 (five then five) expected rolls: " << e2 << "\n";
    cout << "Alice should play: Game 1 (five then six)\n";
    return 0;
}
