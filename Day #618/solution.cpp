// Pancake sort using only reverse(lst,i,j): flip current max to front, then to its place.
// Time: O(n^2) comparisons, O(n) flips, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

void reverse_(vector<int>& lst, int i, int j) {
    while (i < j) { swap(lst[i], lst[j]); i++; j--; }
}

void pancakeSort(vector<int>& lst) {
    for (int size = (int)lst.size(); size > 1; size--) {
        int maxIdx = 0;
        for (int k = 1; k < size; k++) if (lst[k] > lst[maxIdx]) maxIdx = k;
        if (maxIdx != size - 1) {
            if (maxIdx != 0) reverse_(lst, 0, maxIdx); // bring max to front
            reverse_(lst, 0, size - 1);                // flip to final position
        }
    }
}

int main() {
    vector<int> lst = {3, 1, 4, 1, 5, 9, 2, 6};
    pancakeSort(lst);
    cout << "[";
    for (size_t i = 0; i < lst.size(); i++) {
        if (i) cout << ", ";
        cout << lst[i];
    }
    cout << "]\n";
    return 0;
}
