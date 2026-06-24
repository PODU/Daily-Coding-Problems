# Day 1709: Non-decreasing with <=1 change: single pass, on violation greedily lower a[i-1] or raise a[i]. O(n).
def check_possibility(a):
    a = list(a)
    cnt = 0
    for i in range(1, len(a)):
        if a[i - 1] > a[i]:
            cnt += 1
            if cnt > 1:
                return False
            if i < 2 or a[i - 2] <= a[i]:
                a[i - 1] = a[i]
            else:
                a[i] = a[i - 1]
    return True


if __name__ == "__main__":
    print(str(check_possibility([10, 5, 7])).lower())
    print(str(check_possibility([10, 5, 1])).lower())
