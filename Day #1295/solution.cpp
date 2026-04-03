// Day 1295: Fixed-size log of last N order ids via circular buffer.
// record and get_last are both O(1) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

class OrderLog {
    vector<long long> buf;
    int n, head = 0, count = 0; // head = index of next write
public:
    OrderLog(int N) : buf(N), n(N) {}
    void record(long long id) {
        buf[head] = id;
        head = (head + 1) % n;
        if (count < n) count++;
    }
    // i-th last: 1 = most recent
    long long get_last(int i) {
        int idx = ((head - i) % n + n) % n;
        return buf[idx];
    }
};

int main() {
    OrderLog log(3);
    log.record(10);
    log.record(20);
    log.record(30);
    cout << log.get_last(1) << endl; // 30
    cout << log.get_last(2) << endl; // 20
    log.record(40); // evicts 10
    cout << log.get_last(1) << endl; // 40
    cout << log.get_last(3) << endl; // 20
    return 0;
}
