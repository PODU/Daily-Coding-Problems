// Day 964: Order log keeping last N ids with O(1) record/get_last.
// Approach: fixed-size circular buffer. Time O(1) per op, Space O(N).
#include <bits/stdc++.h>
using namespace std;

class OrderLog {
    vector<int> buf;
    int n, count = 0; // count = total recorded
public:
    OrderLog(int N): buf(N), n(N) {}
    void record(int orderId) { buf[count % n] = orderId; count++; }
    // i-th last element: i=1 is most recent
    int getLast(int i) { return buf[(count - i) % n]; }
};

int main() {
    OrderLog log(3);
    log.record(10);
    log.record(20);
    log.record(30);
    log.record(40); // evicts 10; buffer holds 20,30,40
    cout << log.getLast(1) << endl; // 40
    cout << log.getLast(2) << endl; // 30
    cout << log.getLast(3) << endl; // 20
    return 0;
}
