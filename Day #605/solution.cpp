// h-index via counting/bucket sort: bucket citations capped at N, scan from high
// to low accumulating paper counts. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int hIndex(const vector<int>& citations) {
    int n = citations.size();
    vector<int> bucket(n + 1, 0);
    for (int c : citations) bucket[min(c, n)]++;
    int total = 0;
    for (int h = n; h >= 0; --h) {
        total += bucket[h];
        if (total >= h) return h;
    }
    return 0;
}

int main() {
    vector<int> citations = {4, 3, 0, 1, 5};
    cout << hIndex(citations) << endl;
    return 0;
}
