from itertools import combinations_with_replacement

print(*(''.join(item) for item in combinations_with_replacement(input("要素を空白区切りで：").split(), int(input("桁数：")))), sep='\n')
