// Markov chain simulation: seeded mt19937 RNG; O(steps*states) time O(states^2) space
// Counts state arrived at after each step (not initial state); total counts = num_steps = 5000
// Exact counts vary by seed; approx distribution: ~60% a, ~33% b, ~7% c
#include <bits/stdc++.h>
using namespace std;

int main() {
    map<char, vector<pair<char, double>>> trans;
    vector<tuple<char, char, double>> transitions = {
        {'a','a',0.9},  {'a','b',0.075}, {'a','c',0.025},
        {'b','a',0.15}, {'b','b',0.8},   {'b','c',0.05},
        {'c','a',0.25}, {'c','b',0.25},  {'c','c',0.5}
    };
    for (auto& [f, t, p] : transitions)
        trans[f].push_back({t, p});

    mt19937 rng(42);
    uniform_real_distribution<double> dist(0.0, 1.0);

    char state = 'a';
    map<char, int> counts = {{'a', 0}, {'b', 0}, {'c', 0}};
    const int num_steps = 5000;

    for (int i = 0; i < num_steps; i++) {
        double r = dist(rng);
        double cumulative = 0.0;
        for (auto& [to, prob] : trans[state]) {
            cumulative += prob;
            if (r < cumulative) {
                state = to;
                break;
            }
        }
        counts[state]++;
    }

    cout << "{ 'a': " << counts['a']
         << ", 'b': " << counts['b']
         << ", 'c': " << counts['c'] << " }" << "\n";
    return 0;
}
