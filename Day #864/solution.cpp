// Day 864: Minimum rescue boats (<=2 people, total weight <= k).
// Approach: sort, greedily pair lightest with heaviest using two pointers.
// Time: O(n log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int numRescueBoats(vector<int> w, int k) {
    sort(w.begin(), w.end());
    int i = 0, j = w.size() - 1, boats = 0;
    while (i <= j) {
        if (w[i] + w[j] <= k) i++;
        j--;
        boats++;
    }
    return boats;
}

int main() {
    cout << numRescueBoats({100, 200, 150, 80}, 200) << endl; // 3
    return 0;
}
