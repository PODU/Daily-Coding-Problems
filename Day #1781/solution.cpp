// Unit converter as a graph: addRelation stores 1 a = factor b (edge a->b, b->a=1/factor).
// convert does BFS multiplying factors along the path. Time O(V+E) per query, Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

class UnitConverter {
    unordered_map<string, vector<pair<string,double>>> adj;
public:
    void addRelation(const string& a, const string& b, double factor) {
        adj[a].push_back({b, factor});
        adj[b].push_back({a, 1.0/factor});
    }
    // returns {found, value}
    pair<bool,double> convert(double qty, const string& from, const string& to) {
        if (from == to) return {true, qty};
        if (!adj.count(from) || !adj.count(to)) return {false, 0};
        unordered_map<string,double> dist;
        queue<string> q;
        q.push(from); dist[from] = qty;
        while (!q.empty()) {
            string u = q.front(); q.pop();
            for (auto& e : adj[u]) {
                if (!dist.count(e.first)) {
                    dist[e.first] = dist[u] * e.second;
                    if (e.first == to) return {true, dist[e.first]};
                    q.push(e.first);
                }
            }
        }
        return {false, 0};
    }
};

int main() {
    UnitConverter uc;
    uc.addRelation("inches", "foot", 1.0/12.0); // 12 inches = 1 foot
    uc.addRelation("feet", "yard", 1.0/3.0);    // 3 feet = 1 yard
    uc.addRelation("yards", "chain", 1.0/22.0); // 22 yards = 1 chain
    // bridge equivalent singular/plural unit names
    uc.addRelation("foot", "feet", 1.0);
    uc.addRelation("yard", "yards", 1.0);

    auto r1 = uc.convert(1, "yard", "inches");
    auto r2 = uc.convert(1, "chain", "inches");
    auto r3 = uc.convert(1, "chain", "feet");
    cout << "1 yard = " << r1.second << " inches\n";
    cout << "1 chain = " << r2.second << " inches\n";
    cout << "1 chain = " << r3.second << " feet\n";
    return 0;
}
