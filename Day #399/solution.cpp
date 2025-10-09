// Partition into 3 contiguous equal-sum parts: greedy prefix cut at target, absorbing trailing zeros. O(n) time, O(n) space.
#include <iostream>
#include <vector>
using namespace std;

bool partition3(const vector<int>& L, vector<vector<int>>& parts) {
    long long total = 0;
    for (int x : L) total += x;
    if (total % 3 != 0) return false;
    long long target = total / 3;
    vector<vector<int>> res;
    vector<int> cur;
    long long running = 0;
    for (size_t i = 0; i < L.size(); ++i) {
        cur.push_back(L[i]);
        running += L[i];
        // close part when sum hits target and the next element is non-zero (so zeros stay attached)
        if (res.size() < 2 && running == target && (i + 1 == L.size() || L[i + 1] != 0)) {
            res.push_back(cur);
            cur.clear();
            running = 0;
        }
    }
    res.push_back(cur);
    if (res.size() != 3) return false;
    for (auto& p : res) {
        long long s = 0;
        for (int x : p) s += x;
        if (s != target) return false;
    }
    parts = res;
    return true;
}

void printParts(const vector<vector<int>>& parts) {
    cout << "[";
    for (size_t i = 0; i < parts.size(); ++i) {
        cout << "[";
        for (size_t j = 0; j < parts[i].size(); ++j) {
            cout << parts[i][j];
            if (j + 1 < parts[i].size()) cout << ", ";
        }
        cout << "]";
        if (i + 1 < parts.size()) cout << ", ";
    }
    cout << "]\n";
}

int main() {
    vector<int> L = {3, 5, 8, 0, 8};
    vector<vector<int>> parts;
    if (partition3(L, parts)) printParts(parts);
    else cout << "None\n";
    return 0;
}
