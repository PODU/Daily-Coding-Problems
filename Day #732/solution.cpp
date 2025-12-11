// Day 732: Minimum boats (each holds <=2 people, weight limit k).
// Approach: Sort; two pointers pair lightest with heaviest when they fit.
// Time: O(n log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int numBoats(vector<int> w, int k) {
    sort(w.begin(), w.end());
    int i = 0, j = w.size() - 1, boats = 0;
    while (i <= j) {
        if (w[i] + w[j] <= k) i++;   // lightest joins heaviest
        j--;                          // heaviest always leaves
        boats++;
    }
    return boats;
}

int main() {
    vector<int> weights = {100, 200, 150, 80};
    cout << numBoats(weights, 200) << "\n"; // 3
    return 0;
}
