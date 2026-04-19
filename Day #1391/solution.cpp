// Von Neumann fair coin from a biased toss: toss twice, (0,1)->0, (1,0)->1, else retry. O(1) expected per fair toss.
#include <bits/stdc++.h>
using namespace std;

mt19937 rng(12345);
uniform_real_distribution<double> uni(0.0, 1.0);

int toss_biased() { return uni(rng) < 0.3 ? 1 : 0; }

int fair_toss() {
    while (true) {
        int a = toss_biased(), b = toss_biased();
        if (a == 0 && b == 1) return 0;
        if (a == 1 && b == 0) return 1;
    }
}

int main() {
    int n = 100000, heads = 0;
    for (int i = 0; i < n; i++) heads += fair_toss();
    printf("heads fraction: %.2f\n", (double)heads / n);
    return 0;
}
