from itertools import combinations_with_replacement

n = int(input("桁数："))
ls = input("要素を空白区切りで：").split()  

for item in combinations_with_replacement(ls, n):
    print(''.join(item))