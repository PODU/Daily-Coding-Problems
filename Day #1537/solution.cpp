// Simulate a Markov chain for num_steps and count visits per state.
// Time O(num_steps * outdegree), Space O(states + transitions).
#include <bits/stdc++.h>
using namespace std;

map<string,int> runChain(string start, int numSteps,
        map<string, vector<pair<string,double>>>& trans, unsigned seed) {
    map<string,int> counts;
    mt19937 rng(seed);
    uniform_real_distribution<double> dist(0.0, 1.0);
    string cur = start;
    for (int s = 0; s < numSteps; s++) {
        counts[cur]++;
        double r = dist(rng), acc = 0;
        for (auto& [nxt, p] : trans[cur]) {
            acc += p;
            if (r <= acc) { cur = nxt; break; }
        }
    }
    return counts;
}

int main() {
    map<string, vector<pair<string,double>>> trans = {
        {"a", {{"a",0.9},{"b",0.075},{"c",0.025}}},
        {"b", {{"a",0.15},{"b",0.8},{"c",0.05}}},
        {"c", {{"a",0.25},{"b",0.25},{"c",0.5}}}
    };
    auto counts = runChain("a", 5000, trans, 42);
    cout << "{ ";
    bool first = true;
    for (auto& [k, v] : counts) {
        if (!first) cout << ", ";
        cout << "'" << k << "': " << v;
        first = false;
    }
    cout << " }\n";
    return 0;
}
