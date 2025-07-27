// Single number where others appear 3x: ones/twos bit-counting state machine.
// O(N) time, O(1) space. Works for negatives via two's-complement int.
#include <iostream>
#include <vector>
using namespace std;

int singleNumber(const vector<int>& nums) {
    int ones = 0, twos = 0;
    for (int x : nums) {
        ones = (ones ^ x) & ~twos;
        twos = (twos ^ x) & ~ones;
    }
    return ones;
}

int main() {
    cout << singleNumber({6, 1, 3, 3, 3, 6, 6}) << endl;
    return 0;
}
