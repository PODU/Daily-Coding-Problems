// Day 1842: Majority / most-frequent element via a frequency count.
// (Equals the strict majority element whenever one exists.) Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int majority(const vector<int>& a) {
    unordered_map<int, int> freq;
    int best = a.empty() ? 0 : a[0], bestCount = 0;
    for (int x : a) {
        int c = ++freq[x];
        if (c > bestCount) { bestCount = c; best = x; }
    }
    return best;
}

int main() {
    cout << majority({1, 2, 1, 1, 3, 4, 0}) << "\n";  // 1
}
