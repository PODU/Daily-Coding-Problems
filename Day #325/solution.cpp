// Unit conversion via weighted graph; addConversion adds a<->b edges, convert does BFS multiplying ratios.
// Time: O(V+E) per query, Space: O(V+E).
#include <bits/stdc++.h>
using namespace std;

struct UnitConverter {
    unordered_map<string, vector<pair<string,double>>> adj;
    void addConversion(const string& a, const string& b, double ratio) {
        adj[a].push_back({b, ratio});
        adj[b].push_back({a, 1.0/ratio});
    }
    double convert(double value, const string& from, const string& to) {
        if (from == to) return value;
        unordered_map<string,double> dist;
        queue<string> q;
        q.push(from); dist[from] = value;
        while (!q.empty()) {
            string u = q.front(); q.pop();
            for (auto& e : adj[u]) {
                if (!dist.count(e.first)) {
                    dist[e.first] = dist[u] * e.second;
                    if (e.first == to) return dist[e.first];
                    q.push(e.first);
                }
            }
        }
        return dist.count(to) ? dist[to] : nan("");
    }
};

int main() {
    UnitConverter uc;
    uc.addConversion("foot", "inch", 12);
    uc.addConversion("yard", "foot", 3);
    uc.addConversion("chain", "yard", 22);
    double r = uc.convert(1, "yard", "inch");
    cout << "1 yard = " << fixed << setprecision(1) << r << " inch" << endl;
    return 0;
}
