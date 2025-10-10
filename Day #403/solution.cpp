// rand7 from rand5 via rejection sampling: idx=(rand5-1)*5+rand5 in 1..25, reject>21,
// return (idx-1)%7+1. O(1) expected calls. Space O(1).
#include <iostream>
#include <random>
using namespace std;

mt19937 gen(12345);
uniform_int_distribution<int> d5(1, 5);
int rand5() { return d5(gen); }

int rand7() {
    while (true) {
        int idx = (rand5() - 1) * 5 + rand5(); // uniform 1..25
        if (idx <= 21) return (idx - 1) % 7 + 1;
    }
}

int main() {
    const int N = 70000;
    int counts[8] = {0};
    for (int i = 0; i < N; ++i) counts[rand7()]++;
    for (int v = 1; v <= 7; ++v) {
        cout << v << ": " << counts[v] << " ";
        for (int b = 0; b < counts[v] / 250; ++b) cout << "#";
        cout << "\n";
    }
    return 0;
}
