// Markov chain simulation: build cumulative transition table, draw uniform RNG per step.
// Result is stochastic/approximate (fixed seed for reproducibility). Time O(steps), Space O(states^2).
#include <bits/stdc++.h>
using namespace std;

int main() {
    map<char, vector<pair<double,char>>> table;
    vector<tuple<char,char,double>> transitions = {
        {'a','a',0.9},{'a','b',0.075},{'a','c',0.025},
        {'b','a',0.15},{'b','b',0.8},{'b','c',0.05},
        {'c','a',0.25},{'c','b',0.25},{'c','c',0.5}
    };
    // build cumulative probabilities per state
    map<char, vector<pair<double,char>>> raw;
    for (auto& t : transitions) raw[get<0>(t)].push_back({get<2>(t), get<1>(t)});
    for (auto& kv : raw) {
        double cum = 0.0;
        for (auto& p : kv.second) { cum += p.first; table[kv.first].push_back({cum, p.second}); }
    }

    char start = 'a';
    int num_steps = 5000;
    map<char,long long> counts = {{'a',0},{'b',0},{'c',0}};

    mt19937 rng(12345);
    uniform_real_distribution<double> dist(0.0, 1.0);

    char state = start;
    for (int i = 0; i < num_steps; i++) {
        double r = dist(rng);
        for (auto& p : table[state]) {
            if (r < p.first) { state = p.second; break; }
        }
        counts[state]++;
    }

    cout << "{'a': " << counts['a'] << ", 'b': " << counts['b'] << ", 'c': " << counts['c'] << "}" << endl;
    return 0;
}
