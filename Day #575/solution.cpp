// Day 575: 2D iterator over an array of arrays without flattening/cloning.
// Track (row, col); advance over empty rows. next/has_next amortized O(1).
#include <iostream>
#include <vector>
#include <stdexcept>
using namespace std;

class Iterator2D {
    const vector<vector<int>>& data;
    size_t row = 0, col = 0;
    void skipEmpty() {
        while (row < data.size() && col >= data[row].size()) { row++; col = 0; }
    }
public:
    explicit Iterator2D(const vector<vector<int>>& d) : data(d) { skipEmpty(); }
    bool has_next() { return row < data.size(); }
    int next() {
        if (!has_next()) throw out_of_range("no more elements");
        int v = data[row][col++];
        skipEmpty();
        return v;
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
    cout << "\n"; // 1, 2, 3, 4, 5, 6
    return 0;
}
