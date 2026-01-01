// Longest subarray with all distinct elements.
// Sliding window with last-seen index map. Time: O(N), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

int longestDistinct(const vector<int>& a) {
    unordered_map<int,int> last;
    int best = 0, start = 0;
    for (int i = 0; i < (int)a.size(); ++i) {
        auto it = last.find(a[i]);
        if (it != last.end() && it->second >= start) start = it->second + 1;
        last[a[i]] = i;
        best = max(best, i - start + 1);
    }
    return best;
}

int main() {
    vector<int> a = {5, 1, 3, 5, 2, 3, 4, 1};
    cout << longestDistinct(a) << endl;
    return 0;
}
