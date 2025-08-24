// 2D iterator: store outer/inner indices, advance() skips empty subarrays. O(1) amortized per next/has_next, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

class Iterator2D {
    const vector<vector<int>>& data;
    size_t row = 0, col = 0;
    void advance() {
        while (row < data.size() && col >= data[row].size()) { row++; col = 0; }
    }
public:
    Iterator2D(const vector<vector<int>>& d) : data(d) { advance(); }
    bool has_next() { advance(); return row < data.size(); }
    int next() {
        if (!has_next()) throw runtime_error("no more elements");
        return data[row][col++];
    }
};

int main() {
    vector<vector<int>> arr = {{1, 2}, {3}, {}, {4, 5, 6}};
    Iterator2D it(arr);
    bool first = true;
    while (it.has_next()) {
        if (!first) cout << ", ";
        cout << it.next();
        first = false;
    }
    cout << endl;
    return 0;
}
