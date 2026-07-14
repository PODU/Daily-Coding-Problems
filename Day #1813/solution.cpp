// Last-N order log via fixed-size circular buffer.
// record: O(1), get_last(i): O(1). Space: O(N).
#include <bits/stdc++.h>
using namespace std;

class OrderLog {
    vector<long long> buf;
    int n, count = 0, head = 0; // head = index where next write goes
public:
    OrderLog(int N) : buf(N), n(N) {}
    void record(long long id) {
        buf[head] = id;
        head = (head + 1) % n;
        if (count < n) count++;
    }
    // i = 1 -> most recent, i = 2 -> second most recent, ...
    long long get_last(int i) {
        int idx = ((head - i) % n + n) % n;
        return buf[idx];
    }
};

int main() {
    OrderLog log(5);
    for (long long id : {101, 102, 103, 104, 105, 106})
        log.record(id);
    cout << log.get_last(1) << "\n"; // 106 (most recent)
    cout << log.get_last(3) << "\n"; // 104
    return 0;
}
