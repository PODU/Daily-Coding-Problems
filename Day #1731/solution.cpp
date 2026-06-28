// Day 1731: kth row of Pascal's triangle (1-indexed) using O(k) space.
// Build in place via binomial coefficients. Time O(k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

vector<long long> pascalRow(int k) {
    vector<long long> row(k, 1);
    for (int i = 0; i < k; i++)
        // C(k-1, i) = C(k-1, i-1) * (k-i) / i
        if (i > 0) row[i] = row[i - 1] * (k - i) / i;
    return row;
}

int main() {
    int k = 5; // 1-indexed -> row [1,4,6,4,1]
    auto r = pascalRow(k);
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? " " : "\n");
    return 0;
}
