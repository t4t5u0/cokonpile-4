

# class Kenshirou:
#     str: data


def solve(n: int) -> str:
    return bin(n).replace('0', 'た').replace('1', 'あ')[2:]


print(solve(int(input('入力しなさい：'))))