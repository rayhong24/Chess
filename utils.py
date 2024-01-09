from enums import File

def to_coords(move: str) -> (int, int):
    i, j = 8-int(move[1]), File[move[0]].value

    return (i, j)

def coords_to_square(i: int, j: int):
    return str(File(j))+str(8-i)
    