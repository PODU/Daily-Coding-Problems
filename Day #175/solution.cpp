// Markov chain simulation: sample next state via cumulative probabilities, fixed-seed RNG.
// Time O(num_steps * avg_outdegree), Space O(states). (Exact counts depend on RNG.)
#include <iostream>
#include <map>
#include <vector>
#include <string>
#include <random>
using namespace std;

int main() {
    vector<tuple<char, char, double>> transitions = {
        {'a','a',0.9},{'a','b',0.075},{'a','c',0.025},
        {'b','a',0.15},{'b','b',0.8},{'b','c',0.05},
        {'c','a',0.25},{'c','b',0.25},{'c','c',0.5},
    };
    map<char, vector<pair<char, double>>> trans;
    for (auto& [src, dst, prob] : transitions) trans[src].push_back({dst, prob});

    mt19937 rng(42);
    uniform_real_distribution<double> dist(0.0, 1.0);

    char start = 'a';
    int num_steps = 5000;
    map<char, int> counts;
    char state = start;
    for (int i = 0; i < num_steps; i++) {
        counts[state]++;
        double r = dist(rng);
        double cum = 0.0;
        for (auto& [dst, prob] : trans[state]) {
            cum += prob;
            if (r < cum) { state = dst; break; }
        }
    }
    cout << "{'a': " << counts['a'] << ", 'b': " << counts['b']
         << ", 'c': " << counts['c'] << "}" << endl;
    return 0;
}
