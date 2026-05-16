// Boat rescue: min boats, <=2 people each, weight limit k.
// Greedy two-pointer: sort, pair lightest with heaviest if sum<=k. O(n log n) time, O(1) extra.
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int numRescueBoats(vector<int> w, int k) {
    sort(w.begin(), w.end());
    int lo = 0, hi = (int)w.size() - 1, boats = 0;
    while (lo <= hi) {
        if (w[lo] + w[hi] <= k) lo++;
        hi--;
        boats++;
    }
    return boats;
}

int main() {
    vector<int> w = {100, 200, 150, 80};
    cout << numRescueBoats(w, 200) << "\n"; // expected 3
    return 0;
}
