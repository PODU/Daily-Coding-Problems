// Day 970: Space-efficient SparseArray storing only non-zero entries.
// Approach: hash map of index->value, default 0. Time O(1) avg per op, Space O(#nonzero).
#include <bits/stdc++.h>
using namespace std;

class SparseArray {
    unordered_map<int, int> m;
    int size;
public:
    void init(const vector<int>& arr, int sz) {
        size = sz;
        m.clear();
        for (int i = 0; i < sz; ++i) if (arr[i] != 0) m[i] = arr[i];
    }
    void set(int i, int val) {
        if (i < 0 || i >= size) throw out_of_range("index");
        if (val == 0) m.erase(i); else m[i] = val;
    }
    int get(int i) {
        if (i < 0 || i >= size) throw out_of_range("index");
        auto it = m.find(i);
        return it == m.end() ? 0 : it->second;
    }
};

int main() {
    SparseArray sa;
    sa.init({0, 0, 5, 0, 0, 0, 9, 0}, 8);
    cout << sa.get(2) << endl; // 5
    cout << sa.get(3) << endl; // 0
    sa.set(3, 7);
    cout << sa.get(3) << endl; // 7
    sa.set(2, 0);
    cout << sa.get(2) << endl; // 0
    return 0;
}
