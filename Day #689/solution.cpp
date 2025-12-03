// Unit conversion as a weighted graph; convert via BFS multiplying edge ratios.
// add_conversion(a, b, r): 1 a = r b. Time O(V + E) per query, Space O(V + E).
#include <bits/stdc++.h>
using namespace std;

class UnitConverter {
    unordered_map<string, unordered_map<string, double>> graph;
public:
    void addConversion(const string& frm, const string& to, double factor) {
        // 1 frm = factor to
        graph[frm][to] = factor;
        graph[to][frm] = 1.0 / factor;
    }

    // returns true and sets out if convertible
    bool convert(double qty, const string& frm, const string& to, double& out) {
        if (frm == to) { out = qty; return true; }
        if (!graph.count(frm) || !graph.count(to)) return false;
        unordered_set<string> visited{frm};
        queue<pair<string, double>> q;
        q.push({frm, 1.0});
        while (!q.empty()) {
            string unit = q.front().first;
            double ratio = q.front().second;
            q.pop();
            if (unit == to) { out = qty * ratio; return true; }
            for (unordered_map<string, double>::iterator it = graph[unit].begin();
                 it != graph[unit].end(); ++it) {
                if (!visited.count(it->first)) {
                    visited.insert(it->first);
                    q.push(make_pair(it->first, ratio * it->second));
                }
            }
        }
        return false;
    }
};

int main() {
    UnitConverter uc;
    uc.addConversion("foot", "inch", 12);   // 12 inches = 1 foot
    uc.addConversion("yard", "foot", 3);    // 3 feet = 1 yard
    uc.addConversion("chain", "yard", 22);  // 22 yards = 1 chain

    double r1, r2;
    uc.convert(1, "chain", "inch", r1);
    uc.convert(1, "yard", "inch", r2);
    cout << "1 chain = " << (long long)llround(r1) << " inches" << endl;
    cout << "1 yard = " << (long long)llround(r2) << " inches" << endl;
    return 0;
}
