// Boats: <=2 people, weight limit k, minimize boats. Sort + two pointers.
// Pair lightest with heaviest if they fit, else heaviest alone. Time O(n log n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int numBoats(vector<int> w, int k) {
    sort(w.begin(), w.end());
    int l = 0, h = (int)w.size() - 1, boats = 0;
    while (l <= h) {
        if (w[l] + w[h] <= k) l++;
        h--;
        boats++;
    }
    return boats;
}

int main() {
    cout << numBoats({100, 200, 150, 80}, 200) << endl;
    return 0;
}
