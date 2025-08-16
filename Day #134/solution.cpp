// Day 134: SparseArray storing only non-zero entries in a hash map.
// init O(n) once, set/get O(1) average. Space O(#nonzero).
#include <bits/stdc++.h>
using namespace std;

struct SparseArray {
    unordered_map<int, int> data;
    int size;
    void init(const vector<int>& arr, int sz) {
        size = sz;
        data.clear();
        for (int i = 0; i < (int)arr.size() && i < sz; i++)
            if (arr[i] != 0) data[i] = arr[i];
    }
    void set(int i, int val) {
        if (i < 0 || i >= size) throw out_of_range("index");
        if (val == 0) data.erase(i);
        else data[i] = val;
    }
    int get(int i) {
        if (i < 0 || i >= size) throw out_of_range("index");
        auto it = data.find(i);
        return it == data.end() ? 0 : it->second;
    }
};

int main() {
    SparseArray sa;
    sa.init({0, 0, 7, 0, 0, 0, 3, 0}, 8);
    cout << sa.get(2) << endl; // 7
    cout << sa.get(0) << endl; // 0
    sa.set(0, 5);
    cout << sa.get(0) << endl; // 5
    sa.set(2, 0);
    cout << sa.get(2) << endl; // 0
    return 0;
}
