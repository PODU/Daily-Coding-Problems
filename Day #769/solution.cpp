// Day 769: Expected rolls for "5 then 6" vs "5 then 5" stopping games.
// Exact via 2-state Markov chains (E1=36, E2=42) plus Monte-Carlo check. O(trials).
#include <bits/stdc++.h>
using namespace std;

double simulate(int second, long trials, mt19937& rng) {
    uniform_int_distribution<int> d(1, 6);
    long total = 0;
    for (long t = 0; t < trials; t++) {
        int prev = 0, rolls = 0;
        while (true) {
            int r = d(rng); rolls++;
            if (prev == 5 && r == second) break;
            prev = r;
        }
        total += rolls;
    }
    return (double)total / trials;
}

int main() {
    mt19937 rng(12345);
    long trials = 500000;
    cout << "Game 1 (5 then 6): exact=36, simulated=" << simulate(6, trials, rng) << "\n";
    cout << "Game 2 (5 then 5): exact=42, simulated=" << simulate(5, trials, rng) << "\n";
    cout << "Alice should play Game 1 (5 then 6); it has the lower expected cost.\n";
    return 0;
}
