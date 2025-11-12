// Group users into a set per site; for every site pair compute Jaccard = |A&B|/|A|B|.
// Sort by similarity DESC, tie-break by pair lexicographically; take top k. Time O(P^2 * U).
#include <iostream>
#include <vector>
#include <string>
#include <map>
#include <set>
#include <algorithm>
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
    sort(names.begin(), names.end());

    struct Cand { double sim; string x, y; };
    vector<Cand> cands;
    for (size_t i = 0; i < names.size(); i++) {
        for (size_t j = i + 1; j < names.size(); j++) {
            const auto& A = sites[names[i]];
            const auto& B = sites[names[j]];
            int inter = 0;
            for (int u : A) if (B.count(u)) inter++;
            int uni = (int)A.size() + (int)B.size() - inter;
            double sim = uni == 0 ? 0.0 : (double)inter / uni;
            cands.push_back({sim, names[i], names[j]});
        }
    }
    sort(cands.begin(), cands.end(), [](const Cand& a, const Cand& b){
        if (a.sim != b.sim) return a.sim > b.sim;
        if (a.x != b.x) return a.x < b.x;
        return a.y < b.y;
    });

    cout << "[";
    for (int i = 0; i < k && i < (int)cands.size(); i++) {
        if (i) cout << ", ";
        cout << "('" << cands[i].x << "', '" << cands[i].y << "')";
    }
    cout << "]\n";
    return 0;
}
