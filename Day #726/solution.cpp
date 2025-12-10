// Day 726: Singleton holding TWO instances; alternate on even/odd getInstance() calls.
// Approach: Static counter; odd call -> instance #2, even call -> instance #1.
// Time: O(1) per call, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

class DualSingleton {
    int id;
    DualSingleton(int i) : id(i) {}
    static int callCount;
public:
    int getId() const { return id; }
    static DualSingleton& getInstance() {
        static DualSingleton first(1), second(2);
        callCount++;
        return (callCount % 2 == 0) ? first : second; // even -> first, odd -> second
    }
};
int DualSingleton::callCount = 0;

int main() {
    for (int i = 1; i <= 4; i++)
        cout << "Call " << i << ": instance " << DualSingleton::getInstance().getId() << "\n";
    // Call 1: instance 2, Call 2: instance 1, Call 3: instance 2, Call 4: instance 1
    return 0;
}
