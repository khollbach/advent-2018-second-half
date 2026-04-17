n = 940

sum_of_factors = 0
for i in range(1, n+1):
    for j in range(1, n+1):
        if i * j == n:
            print(i, j)
            sum_of_factors += i
print(sum_of_factors)

n = 0x00a1002c

sum_of_factors = 0
for i in range(1, n+1):
    j = n // i
    if i * j == n:
        print(i, j)
        sum_of_factors += i
print(sum_of_factors)
