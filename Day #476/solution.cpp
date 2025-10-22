// Find duplicate in array of n+1 ints from 1..n using a visited/seen set.
// Time O(n), Space O(n). (Floyd's cycle detection is an O(1)-space alternative.)
#include <bits/stdc++.h>
using namespace std;

int findDuplicate(const vector<int>& a) {
    vector<bool> seen(a.size() + 1, false);
    for (int x : a) {
        if (seen[x]) return x;
        seen[x] = true;
    }
    return -1;
}

int main() {
    vector<int> a = {1, 3, 4, 2, 2};
    cout << findDuplicate(a) << "\n";
}
