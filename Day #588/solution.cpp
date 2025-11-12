// Day 588: Space-efficient SparseArray backed by a hash map of non-zero entries.
// Approach: store only non-zero indices. Time O(1) avg per op, Space O(#nonzero).
#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

class SparseArray {
    unordered_map<int, int> data;
    int size;
public:
    void init(const vector<int>& arr, int sz) {
        size = sz;
        data.clear();
        for (int i = 0; i < (int)arr.size() && i < sz; ++i)
            if (arr[i] != 0) data[i] = arr[i];
    }
    void set(int i, int val) {
        if (i < 0 || i >= size) throw out_of_range("index out of range");
        if (val == 0) data.erase(i);
        else data[i] = val;
    }
    int get(int i) {
        if (i < 0 || i >= size) throw out_of_range("index out of range");
        auto it = data.find(i);
        return it == data.end() ? 0 : it->second;
    }
};

int main() {
    SparseArray sa;
    sa.init({0, 0, 0, 5, 0, 0, 9, 0}, 8);
    cout << "get(3) = " << sa.get(3) << endl;   // 5
    cout << "get(6) = " << sa.get(6) << endl;   // 9
    cout << "get(0) = " << sa.get(0) << endl;   // 0
    sa.set(1, 42);
    cout << "after set(1,42), get(1) = " << sa.get(1) << endl; // 42
    sa.set(3, 0);
    cout << "after set(3,0), get(3) = " << sa.get(3) << endl;  // 0
    return 0;
}
