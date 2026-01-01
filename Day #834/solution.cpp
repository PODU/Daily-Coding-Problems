// Minimum swaps to seat N couples side by side.
// Greedy: partner of p is p^1; swap mismatched partner into place. Time: O(N), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

int minSwaps(vector<int> row) {
    int n = row.size();
    vector<int> pos(n);
    for (int i = 0; i < n; ++i) pos[row[i]] = i;
    int swaps = 0;
    for (int i = 0; i < n; i += 2) {
        int partner = row[i] ^ 1;
        if (row[i + 1] != partner) {
            int j = pos[partner];
            swap(pos[row[i + 1]], pos[row[j]]);
            swap(row[i + 1], row[j]);
            ++swaps;
        }
    }
    return swaps;
}

int main() {
    vector<int> row = {0, 2, 1, 3};
    cout << minSwaps(row) << endl;
    return 0;
}
