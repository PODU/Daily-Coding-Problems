// Day 946: kth row of Pascal's triangle (1-indexed) using O(k) space.
// In-place update of a single row, right-to-left. Time O(k^2), Space O(k).
#include <bits/stdc++.h>
using namespace std;

vector<long long> pascalRow(int k) {
    vector<long long> row(1, 1); // row 1 = [1]
    for (int i = 1; i < k; ++i) {
        row.push_back(0);
        for (int j = (int)row.size() - 1; j > 0; --j)
            row[j] += row[j - 1];
    }
    return row;
}

int main() {
    int k = 5; // README example rows -> 5th row
    vector<long long> r = pascalRow(k);
    for (size_t i = 0; i < r.size(); ++i)
        cout << r[i] << (i + 1 < r.size() ? " " : "\n");
    return 0;
}
