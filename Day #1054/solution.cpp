// Day 1054: Dual-instance singleton. Holds two lazily-created instances and an
// alternating counter; getInstance() returns instance[count % 2], then bumps the
// counter. Time O(1) per call, Space O(1).

#include <bits/stdc++.h>
using namespace std;

class DualSingleton {
    int id_;
    DualSingleton(int id) : id_(id) {}
public:
    int id() const { return id_; }
    static DualSingleton& getInstance() {
        static DualSingleton a(1), b(2);
        static int count = 0;
        DualSingleton& chosen = (count % 2 == 0) ? a : b;
        count++;
        return chosen;
    }
};

int main() {
    for (int call = 0; call < 6; call++) {
        const char* kind = (call % 2 == 0) ? "first" : "second";
        const char* parity = (call % 2 == 0) ? "even" : "odd";
        cout << "call " << call << " (" << parity << ") -> " << kind
             << " instance (id=" << DualSingleton::getInstance().id() << ")\n";
    }
    return 0;
}
