// Day 640: Simulate a Markov chain and tally state visits.
// Approach: build outgoing-edge table, draw next state by cumulative prob.
// Time: O(num_steps * avg_out_degree), Space: O(states + edges).
// Note: output is stochastic; counts approximate the README sample (sum 5000).
#include <bits/stdc++.h>
using namespace std;

map<string,int> runMarkov(const string& start,
                          const vector<tuple<string,string,double>>& trans,
                          int steps, unsigned seed) {
    map<string, vector<pair<string,double>>> adj;
    for (auto& t : trans) adj[get<0>(t)].push_back({get<1>(t), get<2>(t)});
    map<string,int> counts;
    mt19937 rng(seed);
    uniform_real_distribution<double> uni(0.0, 1.0);
    string cur = start;
    for (int i = 0; i < steps; i++) {
        counts[cur]++;
        double r = uni(rng), acc = 0;
        for (auto& e : adj[cur]) {
            acc += e.second;
            if (r <= acc) { cur = e.first; break; }
        }
    }
    return counts;
}

int main() {
    vector<tuple<string,string,double>> trans = {
        {"a","a",0.9},{"a","b",0.075},{"a","c",0.025},
        {"b","a",0.15},{"b","b",0.8},{"b","c",0.05},
        {"c","a",0.25},{"c","b",0.25},{"c","c",0.5}
    };
    auto counts = runMarkov("a", trans, 5000, 42);
    cout << "{ ";
    bool first = true;
    for (auto& kv : counts) {
        if (!first) cout << ", ";
        cout << "'" << kv.first << "': " << kv.second;
        first = false;
    }
    cout << " }\n"; // e.g. { 'a': 3012, 'b': 1656, 'c': 332 }
    return 0;
}
