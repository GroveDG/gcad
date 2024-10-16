from pathlib import Path
from typing import List

def ff(n):
    return f"{n:.3f}".rstrip('0').rstrip('.')

def cyclic_pairwise(iterable):
    iterator = iter(iterable)
    first = next(iterator, None)
    a = first

    for b in iterator:
        yield a, b
        a = b
    
    yield a, first

def read_file(filepath: Path) -> List[str]:
    with open(filepath) as file:
        doc = file.read()
    doc = doc.replace("\n", ",")
    return doc.split(",")