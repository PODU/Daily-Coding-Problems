// H-index via counting buckets: bucket papers by min(citation, n), scan from high. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int hIndex(const vector<int>& citations) {
    int n = citations.size();
    vector<int> bucket(n + 1, 0);
    for (int c : citations) bucket[min(c, n)]++;
    int count = 0;
    for (int h = n; h >= 0; --h) {
        count += bucket[h];
        if (count >= h) return h;
    }
    return 0;
}

int main() {
    vector<int> citations = {4, 3, 0, 1, 5};
    cout << hIndex(citations) << "\n";
    return 0;
}
