# Day 1398: Single scan: count drops; on a drop, greedily fix by lowering a[i] or
# raising a[i+1] depending on a[i-1]. >1 drop => false. Time O(n), Space O(1).

def check_possibility(a):
    a = a[:]
    cnt = 0
    for i in range(len(a) - 1):
        if a[i] > a[i + 1]:
            cnt += 1
            if cnt > 1:
                return False
            if i == 0 or a[i - 1] <= a[i + 1]:
                a[i] = a[i + 1]
            else:
                a[i + 1] = a[i]
    return True


if __name__ == "__main__":
    print(str(check_possibility([10, 5, 7])).lower())
    print(str(check_possibility([10, 5, 1])).lower())
