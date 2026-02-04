// Pancake sort using only reverse(lst,i,j): for each end, bring max of prefix to front then flip to its spot.
// O(n^2) comparisons, O(n) reversals, in place. Space O(1).
#include <bits/stdc++.h>
using namespace std;

void reverseRange(vector<int>& lst, int i, int j) {
    while (i < j) { swap(lst[i], lst[j]); i++; j--; }
}

void pancakeSort(vector<int>& lst) {
    int n = lst.size();
    for (int end = n - 1; end > 0; end--) {
        int mi = 0;
        for (int k = 1; k <= end; k++) if (lst[k] > lst[mi]) mi = k;
        if (mi == end) continue;
        if (mi != 0) reverseRange(lst, 0, mi); // bring max to front
        reverseRange(lst, 0, end);             // move max to its final position
    }
}

int main() {
    vector<int> arr = {3, 1, 4, 1, 5, 9, 2, 6};
    pancakeSort(arr);
    cout << "[";
    for (size_t i = 0; i < arr.size(); i++) {
        cout << arr[i];
        if (i + 1 < arr.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
