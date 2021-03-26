from itertools import product

print('',*[f'{n[0]*n[1]:2}'+('\n' if i % 9 == 0 else '') for i, n in enumerate(product(range(1, 10), range(1, 10)), start=1)])