// Day 375: h-index via counting sort.
// Bucket citations (capped at n), then scan h from n down accumulating papers
// with >= h citations; first h with count >= h wins. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int hIndex(const vector<int>& cites) {
    int n = cites.size();
    vector<int> buckets(n + 1, 0);
    for (int c : cites) buckets[min(c, n)]++;
    int total = 0;
    for (int h = n; h >= 0; h--) {
        total += buckets[h];
        if (total >= h) return h;
    }
    return 0;
}

int main() {
    vector<int> cites = {4, 0, 0, 2, 3};
    cout << hIndex(cites) << "\n"; // 2
    return 0;
}
