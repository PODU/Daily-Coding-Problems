// External/bucket sort demo for sorting ~1M ints in [0,1e9] with bounded memory.
// Real answer for data exceeding RAM: external merge sort (split into chunks ->
// sort each chunk on disk -> k-way merge). Here we bucket by range, sort each
// bucket, and concatenate. Time: O(n log(n/k)); Space: O(n) bounded per bucket.
#include <bits/stdc++.h>
using namespace std;

vector<long long> bucketSort(vector<long long> data, long long maxVal, int numBuckets) {
    vector<vector<long long>> buckets(numBuckets);
    long long width = maxVal / numBuckets + 1;
    for (long long x : data) {
        int b = (int)(x / width);
        if (b >= numBuckets) b = numBuckets - 1;
        buckets[b].push_back(x);
    }
    vector<long long> out;
    for (auto& bk : buckets) {
        sort(bk.begin(), bk.end()); // each bucket fits in memory
        for (long long x : bk) out.push_back(x);
    }
    return out;
}

int main() {
    vector<long long> input = {5, 1, 4, 2, 8, 1000000000LL, 0};
    auto sorted = bucketSort(input, 1000000000LL, 16);
    for (size_t i = 0; i < sorted.size(); i++) {
        cout << sorted[i];
        if (i + 1 < sorted.size()) cout << " ";
    }
    cout << "\n";
    return 0;
}
