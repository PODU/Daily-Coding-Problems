// Day 504: Last-N order log via fixed-size circular buffer.
// record O(1), get_last(i) O(1); space O(N).
#include <iostream>
#include <vector>

class OrderLog {
    std::vector<long long> buf;
    int cap;
    int pos = 0;   // index where the next record will be written
    int count = 0; // number of records seen (capped at cap)
public:
    explicit OrderLog(int n) : buf(n), cap(n) {}

    void record(long long orderId) {
        buf[pos] = orderId;
        pos = (pos + 1) % cap;
        if (count < cap) ++count;
    }

    // i = 1 is the most recent.
    long long get_last(int i) const {
        int idx = ((pos - i) % cap + cap) % cap;
        return buf[idx];
    }
};

int main() {
    OrderLog log(5);
    for (long long id : {1, 2, 3, 4, 5, 6, 7}) log.record(id);
    std::cout << "get_last(1) = " << log.get_last(1) << std::endl;
    std::cout << "get_last(2) = " << log.get_last(2) << std::endl;
    std::cout << "get_last(3) = " << log.get_last(3) << std::endl;
    return 0;
}
