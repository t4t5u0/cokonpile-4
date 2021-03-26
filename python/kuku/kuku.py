from itertools import product


for i, n in enumerate(product(x := range(1, 10), x), start=1):
    print(f'{n[0]*n[1]:2}', end='\n' if i % 9 == 0 else ' ')
