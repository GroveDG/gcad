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