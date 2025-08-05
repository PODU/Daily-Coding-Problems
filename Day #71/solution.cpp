// rand5 from rand7 via rejection sampling: call rand7() until <=5. Uniform 1..5. Time O(1) expected, Space O(1).
#include <bits/stdc++.h>
using namespace std;

int rand7() {
    static mt19937 rng(random_device{}());
    static uniform_int_distribution<int> dist(1, 7);
    return dist(rng);
}

int rand5() {
    int x;
    do { x = rand7(); } while (x > 5);
    return x;
}

int main() {
    const int trials = 100000;
    long counts[6] = {0};
    for (int i = 0; i < trials; i++) {
        int v = rand5();
        assert(v >= 1 && v <= 5);
        counts[v]++;
    }
    double expected = trials / 5.0;
    for (int v = 1; v <= 5; v++) {
        assert(fabs(counts[v] - expected) < expected * 0.1);
    }
    cout << "rand5 OK" << endl;
    return 0;
}
