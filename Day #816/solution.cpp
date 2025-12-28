// Branchless select via bitwise mask: y ^ ((-b) & (x ^ y)). O(1) time, O(1) space.
#include <iostream>
using namespace std;

int select(int x, int y, int b) {
    return y ^ ((-b) & (x ^ y));
}

int main() {
    cout << "b=1 -> " << select(5, 9, 1) << "\n";
    cout << "b=0 -> " << select(5, 9, 0) << "\n";
    return 0;
}
