// Top-k similar website pairs by Jaccard similarity of user sets.
// Build per-site user sets, compute Jaccard for all pairs, sort desc (ties alpha), take k.
// Time: O(S^2 * U) for pair intersections, Space: O(S*U).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<string,int>> visits = {
        {"a",1},{"a",3},{"a",5},{"b",2},{"b",6},
        {"c",1},{"c",2},{"c",3},{"c",4},{"c",5},
        {"d",4},{"d",5},{"d",6},{"d",7},
        {"e",1},{"e",3},{"e",5},{"e",6}
    };
    int k = 1;

    map<string, set<int>> sites;
    for (auto& v : visits) sites[v.first].insert(v.second);

    vector<string> names;
    for (auto& s : sites) names.push_back(s.first);

    struct Res { double sim; string a, b; };
    vector<Res> results;
    for (size_t i = 0; i < names.size(); ++i) {
        for (size_t j = i + 1; j < names.size(); ++j) {
            const auto& A = sites[names[i]];
            const auto& B = sites[names[j]];
            vector<int> inter;
            set_intersection(A.begin(), A.end(), B.begin(), B.end(), back_inserter(inter));
            int uni = (int)A.size() + (int)B.size() - (int)inter.size();
            double sim = uni == 0 ? 0.0 : (double)inter.size() / uni;
            results.push_back({sim, names[i], names[j]});
        }
    }
    sort(results.begin(), results.end(), [](const Res& x, const Res& y){
        if (x.sim != y.sim) return x.sim > y.sim;
        if (x.a != y.a) return x.a < y.a;
        return x.b < y.b;
    });

    cout << "[";
    for (int i = 0; i < k && i < (int)results.size(); ++i) {
        if (i) cout << ", ";
        cout << "('" << results[i].a << "', '" << results[i].b << "')";
    }
    cout << "]" << endl;
    return 0;
}
