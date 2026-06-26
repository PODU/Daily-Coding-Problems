// h-index via counting sort: bucket citations capped at N, scan from high to low
// accumulating papers until count >= citation level. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int hIndex(const vector<int>& citations) {
    int n = citations.size();
    vector<int> bucket(n + 1, 0);
    for (int c : citations) bucket[min(c, n)]++;
    int acc = 0;
    for (int h = n; h >= 0; --h) {
        acc += bucket[h];
        if (acc >= h) return h;
    }
    return 0;
}

int main() {
    vector<int> citations = {4, 3, 0, 1, 5};
    cout << hIndex(citations) << "\n";
    return 0;
}
