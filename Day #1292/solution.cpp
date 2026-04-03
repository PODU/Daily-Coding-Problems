// Day 1292: kth (0-indexed) row of Pascal's triangle.
// Update row in place from right to left. O(k) time per element, O(k) space.
#include <bits/stdc++.h>
using namespace std;

vector<long long> pascalRow(int k) {
    vector<long long> row(k + 1, 1);
    for (int i = 1; i <= k; i++)
        for (int j = i - 1; j >= 1; j--)
            row[j] += row[j - 1];
    return row;
}

int main() {
    auto r = pascalRow(4); // row 4 -> 1 4 6 4 1
    for (size_t i = 0; i < r.size(); i++)
        cout << r[i] << (i + 1 < r.size() ? " " : "\n");
    return 0;
}
