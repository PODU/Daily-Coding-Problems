// Day 1584: 2D iterator over array of arrays (no flatten/clone).
// Maintain (outer,inner) indices; advance() skips empty inner arrays. Time: O(1) amortized; Space: O(1).
#include <bits/stdc++.h>
using namespace std;

class Iterator2D {
    const vector<vector<int>>& data;
    size_t outer = 0, inner = 0;
    void skip() { while (outer < data.size() && inner >= data[outer].size()) { outer++; inner = 0; } }
public:
    Iterator2D(const vector<vector<int>>& d) : data(d) { skip(); }
    bool hasNext() { skip(); return outer < data.size(); }
    int next() {
        if (!hasNext()) throw runtime_error("no more elements");
        return data[outer][inner++];
    }
};

int main() {
    vector<vector<int>> v = {{1, 2}, {3}, {}, {4, 5, 6}};
    Iterator2D it(v);
    vector<int> out;
    while (it.hasNext()) out.push_back(it.next());
    for (size_t i = 0; i < out.size(); i++) cout << out[i] << (i + 1 < out.size() ? ", " : "");
    cout << "\n"; // 1, 2, 3, 4, 5, 6
    return 0;
}
