// Day 856: Top k similar website pairs.
// Approach: Jaccard similarity (|A∩B| / |A∪B|) over user sets, take top k pairs.
// Time: O(W^2 * U), Space: O(W * U) where W=#websites, U=avg users.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<string,int>> visits = {
        {"a",1},{"a",3},{"a",5},
        {"b",2},{"b",6},
        {"c",1},{"c",2},{"c",3},{"c",4},{"c",5},
        {"d",4},{"d",5},{"d",6},{"d",7},
        {"e",1},{"e",3},{"e",5},{"e",6}
    };
    int k = 1;

    map<string, set<int>> sites;
    for (auto& v : visits) sites[v.first].insert(v.second);

    vector<string> names;
    for (auto& s : sites) names.push_back(s.first);

    // (similarity, pair)
    vector<pair<double, pair<string,string>>> scored;
    for (size_t i = 0; i < names.size(); i++)
        for (size_t j = i + 1; j < names.size(); j++) {
            auto& A = sites[names[i]];
            auto& B = sites[names[j]];
            int inter = 0;
            for (int u : A) if (B.count(u)) inter++;
            int uni = (int)A.size() + (int)B.size() - inter;
            double sim = uni == 0 ? 0.0 : (double)inter / uni;
            scored.push_back({sim, {names[i], names[j]}});
        }
    sort(scored.begin(), scored.end(), [](auto& x, auto& y){ return x.first > y.first; });

    cout << "[";
    for (int i = 0; i < k && i < (int)scored.size(); i++) {
        if (i) cout << ", ";
        cout << "('" << scored[i].second.first << "', '" << scored[i].second.second << "')";
    }
    cout << "]" << endl;
    return 0;
}
