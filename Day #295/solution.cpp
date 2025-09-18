// Kth row of Pascal's triangle (1-indexed) via iterative binomials in one array. O(k) space, O(k) time.
#include <bits/stdc++.h>
using namespace std;

vector<long long> pascalRow(int k) {
    int n = k - 1; // 0-indexed row number
    vector<long long> row(k, 1);
    for (int r = 1; r < k; ++r) row[r] = row[r-1] * (n - r + 1) / r;
    return row;
}

int main() {
    auto row = pascalRow(5);
    cout << "[";
    for (size_t i = 0; i < row.size(); ++i) cout << row[i] << (i+1<row.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
