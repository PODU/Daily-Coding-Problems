// Unit Converter: graph where edge a->b stores factor (1 a = f b).
// convert() does BFS multiplying factors. Time O(V+E) per query, Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

struct UnitConverter {
    unordered_map<string, vector<pair<string,double>>> g;

    void addUnit(const string& from, const string& to, double factor) {
        g[from].push_back({to, factor});
        g[to].push_back({from, 1.0 / factor});
    }

    // returns value of 'value from' expressed in 'to', or NaN if unreachable.
    double convert(double value, const string& from, const string& to) {
        if (from == to) return value;
        unordered_map<string,double> dist; // factor from 'from' to key
        dist[from] = 1.0;
        queue<string> q; q.push(from);
        while (!q.empty()) {
            string u = q.front(); q.pop();
            for (auto& e : g[u]) {
                if (!dist.count(e.first)) {
                    dist[e.first] = dist[u] * e.second;
                    if (e.first == to) return value * dist[e.first];
                    q.push(e.first);
                }
            }
        }
        return numeric_limits<double>::quiet_NaN();
    }
};

int main() {
    UnitConverter uc;
    uc.addUnit("inch", "foot", 1.0/12);  // 1 inch = 1/12 foot
    uc.addUnit("foot", "yard", 1.0/3);   // 3 feet = 1 yard
    uc.addUnit("yard", "chain", 1.0/22); // 22 yards = 1 chain

    cout << "1 chain = " << (long long)llround(uc.convert(1, "chain", "inch")) << " inches\n";
    cout << "36 inches = " << (long long)llround(uc.convert(36, "inch", "yard")) << " yards\n";
    return 0;
}
