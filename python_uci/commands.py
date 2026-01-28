from dataclasses import dataclass, field
from typing import List, Dict, Any, Optional

@dataclass
class Command:
    name: str
    args: List[str] = field(default_factory=list)
    options: Dict[str, Any] = field(default_factory=dict)
    raw: Optional[str] = None

class CommandParser:
    @staticmethod
    def parse(raw: str) -> Command:
        raw = raw.strip()
        tokens = raw.split()
        if not tokens:
            return Command("", [], {}, raw)

        name = tokens[0]
        i = 1
        args = []
        options = {}

        if name == "position":
            if i < len(tokens) and tokens[i] == "startpos":
                options["fen"] = "startpos"
                i += 1
            elif i < len(tokens) and tokens[i] == "fen":
                i += 1
                fen_parts = []

                while i < len(tokens) and tokens[i] != "moves":
                    fen_parts.append(tokens[i])
                    i += 1
                options["fen"] = " ".join(fen_parts)
            if i < len(tokens) and tokens[i] == "moves":
                i += 1
                moves = tokens[i:]
                options["moves"] = moves
        else:
            args = tokens[1:]

        return Command(name=name, args=args, options=options, raw=raw)