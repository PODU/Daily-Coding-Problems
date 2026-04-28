// Day 1435: Singleton holding two instances; even call -> first, odd call -> second.
// Approach: Two static instances + static call counter, return by parity of count.
// Time: O(1) per call, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

class DualSingleton {
    int id;
    DualSingleton(int i) : id(i) {}
    static int counter;
public:
    int getId() const { return id; }
    static DualSingleton& getInstance() {
        static DualSingleton first(1);
        static DualSingleton second(2);
        int n = counter++;            // 0-indexed call number
        return (n % 2 == 0) ? first : second;
    }
};
int DualSingleton::counter = 0;

int main() {
    for (int i = 0; i < 4; ++i) {
        DualSingleton& inst = DualSingleton::getInstance();
        cout << "Call " << i << " -> instance " << inst.getId() << "\n";
    }
    // Even calls (0,2) -> instance 1, odd calls (1,3) -> instance 2
    return 0;
}
