// Day 1445: Unit converter. Model units as a weighted graph (edge = conversion
// factor) and BFS for a path on query. addUnit O(1); convert O(V+E).
#include <bits/stdc++.h>
using namespace std;

class UnitConverter {
    // graph[a][b] = factor such that 1 a = factor b
    unordered_map<string, unordered_map<string,double>> graph;
public:
    // 1 `from` == factor `to`
    void addUnit(const string& from, const string& to, double factor) {
        graph[from][to] = factor;
        graph[to][from] = 1.0 / factor;
    }

    // returns amount of `to` equal to `value` of `from`, or NaN if no path
    double convert(double value, const string& from, const string& to) {
        if (from == to) return value;
        unordered_map<string,double> dist; // factor from `from`
        queue<string> q; q.push(from); dist[from] = 1.0;
        while (!q.empty()) {
            string u = q.front(); q.pop();
            for (auto it = graph[u].begin(); it != graph[u].end(); ++it) {
                const string& v = it->first;
                double f = it->second;
                if (!dist.count(v)) {
                    dist[v] = dist[u] * f;
                    if (v == to) return value * dist[v];
                    q.push(v);
                }
            }
        }
        return numeric_limits<double>::quiet_NaN();
    }
};

int main() {
    UnitConverter uc;
    uc.addUnit("foot", "inch", 12);
    uc.addUnit("yard", "foot", 3);
    uc.addUnit("chain", "yard", 22);
    cout << (long long)uc.convert(1, "yard", "inch") << "\n"; // 36
    return 0;
}
