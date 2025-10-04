// Day 369: Stock price tracker.
// ts2price maps timestamp->price; a multiset of prices gives min/max in O(log n),
// running sum+count give average in O(1). add/update/remove O(log n).
#include <bits/stdc++.h>
using namespace std;

class StockTracker {
    map<long,int> ts2price;
    multiset<int> prices;
    long long sum = 0;
public:
    void add(long ts, int price) {
        ts2price[ts] = price;
        prices.insert(price);
        sum += price;
    }
    void update(long ts, int price) {
        remove(ts);
        add(ts, price);
    }
    void remove(long ts) {
        auto it = ts2price.find(ts);
        if (it == ts2price.end()) return;
        prices.erase(prices.find(it->second));
        sum -= it->second;
        ts2price.erase(it);
    }
    int maxPrice() { return *prices.rbegin(); }
    int minPrice() { return *prices.begin(); }
    double average() { return (double)sum / prices.size(); }
};

int main() {
    StockTracker s;
    s.add(1, 100); s.add(2, 200); s.add(3, 150);
    cout << "max=" << s.maxPrice() << " min=" << s.minPrice() << " avg=" << s.average() << "\n";
    s.update(2, 50);
    cout << "max=" << s.maxPrice() << " min=" << s.minPrice() << " avg=" << s.average() << "\n";
    s.remove(3);
    cout << "max=" << s.maxPrice() << " min=" << s.minPrice() << " avg=" << s.average() << "\n";
    return 0;
}
