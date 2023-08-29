from enums import File

def get_coords(move: str) -> tuple[int]:
    start_i, start_j = int(move[1]), File[move[0]].value
    new_i, new_j = int(move[3]), File[move[2]].value

    return (start_i, start_j, new_i, new_j)