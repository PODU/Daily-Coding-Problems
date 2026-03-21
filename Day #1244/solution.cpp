// Boats: sort, greedily pair lightest+heaviest (two-pointer). Time O(n log n).
#include <bits/stdc++.h>
using namespace std;

int numBoats(vector<int> w, int k) {
    sort(w.begin(), w.end());
    int i = 0, j = (int)w.size() - 1, boats = 0;
    while (i <= j) {
        if (w[i] + w[j] <= k) ++i;
        --j;
        ++boats;
    }
    return boats;
}

int main() {
    cout << numBoats({100, 200, 150, 80}, 200) << "\n";
    return 0;
}
