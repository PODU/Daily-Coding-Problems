// Binary search without *, /, or bit-shift; midpoint via two-pointer walk (only ++/--).
// Time: O(log N), Space: O(1).
#include <iostream>
#include <vector>
using namespace std;

int midpoint(int lo, int hi) {
    int i = lo, j = hi;
    while (i < j) { i++; j--; }
    return j; // floor((lo+hi)/2) using only ++/--
}

bool contains(const vector<int>& arr, int x) {
    int lo = 0, hi = (int)arr.size() - 1;
    while (lo <= hi) {
        int mid = midpoint(lo, hi);
        if (arr[mid] == x) return true;
        else if (arr[mid] < x) lo = mid + 1;
        else hi = mid - 1;
    }
    return false;
}

int main() {
    vector<int> arr = {1, 3, 5, 7, 9, 11, 13};
    cout << (contains(arr, 7) ? "true" : "false") << "\n";
    cout << (contains(arr, 8) ? "true" : "false") << "\n";
    return 0;
}
