// Day 1430: Space-efficient SparseArray for a mostly-zero array.
// Approach: store only non-zero indices in a hash map. Time: O(1) avg per op, Space: O(#nonzero).
#include <bits/stdc++.h>
using namespace std;

class SparseArray {
    unordered_map<int, int> data;
    int n = 0;
public:
    void init(const vector<int>& arr, int size) {
        n = size;
        data.clear();
        for (int i = 0; i < size; ++i)
            if (arr[i] != 0) data[i] = arr[i];
    }
    void set(int i, int val) {
        if (i < 0 || i >= n) throw out_of_range("index out of bounds");
        if (val == 0) data.erase(i);
        else data[i] = val;
    }
    int get(int i) {
        if (i < 0 || i >= n) throw out_of_range("index out of bounds");
        auto it = data.find(i);
        return it == data.end() ? 0 : it->second;
    }
};

int main() {
    SparseArray sa;
    sa.init({0, 0, 5, 0, 0, 0, 9, 0}, 8);
    cout << sa.get(2) << endl;   // 5
    cout << sa.get(3) << endl;   // 0
    sa.set(3, 7);
    cout << sa.get(3) << endl;   // 7
    sa.set(2, 0);
    cout << sa.get(2) << endl;   // 0
    return 0;
}
