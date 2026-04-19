// Top-k website pairs by Jaccard similarity over user sets; sort desc, tie-break alpha.
// O(W^2 * U) to compare pairs. Output formatted as Python tuple list.
#include <iostream>
#include <vector>
#include <string>
#include <map>
#include <set>
#include <algorithm>
using namespace std;

int main() {
    vector<pair<string,int>> visits = {
        {"a",1},{"a",3},{"a",5},{"b",2},{"b",6},
        {"c",1},{"c",2},{"c",3},{"c",4},{"c",5},
        {"d",4},{"d",5},{"d",6},{"d",7},
        {"e",1},{"e",3},{"e",5},{"e",6}
    };
    int k = 1;

    map<string, set<int>> users;
    for (auto& v : visits) users[v.first].insert(v.second);

    vector<string> sites;
    for (auto& p : users) sites.push_back(p.first);

    struct Res { string a, b; double sim; };
    vector<Res> results;
    for (size_t i = 0; i < sites.size(); ++i)
        for (size_t j = i + 1; j < sites.size(); ++j) {
            auto& A = users[sites[i]]; auto& B = users[sites[j]];
            int inter = 0;
            for (int u : A) if (B.count(u)) ++inter;
            int uni = (int)A.size() + (int)B.size() - inter;
            double sim = uni ? (double)inter / uni : 0.0;
            results.push_back({sites[i], sites[j], sim});
        }

    stable_sort(results.begin(), results.end(), [](const Res& x, const Res& y) {
        if (x.sim != y.sim) return x.sim > y.sim;
        if (x.a != y.a) return x.a < y.a;
        return x.b < y.b;
    });

    cout << "[";
    for (int i = 0; i < k && i < (int)results.size(); ++i) {
        if (i) cout << ", ";
        cout << "('" << results[i].a << "', '" << results[i].b << "')";
    }
    cout << "]\n";
    return 0;
}
