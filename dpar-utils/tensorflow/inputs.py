import re

from data import Embeddings

MORPH_EXPR = re.compile("CHAR (\\d+) (\\d+)")

class Inputs:
    def __init__(self, filename):
        prefix_lens = set()
        suffix_lens = set()

        with open(filename, "r") as f:
            contents = f.read()

            for match in re.finditer(MORPH_EXPR, contents):
                prefix_len = int(match.group(1))
                suffix_len = int(match.group(2))

                prefix_lens.add(prefix_len)
                suffix_lens.add(suffix_len)

        if len(prefix_lens) > 1 or len(suffix_lens) > 1:
            raise RuntimeError("Model does not support inputs with differing prefix/suffix lengths")

        self._prefix_len = prefix_lens.pop()
        self._suffix_len = suffix_lens.pop()

    @property
    def prefix_len(self):
        return self._prefix_len

    @property
    def suffix_len(self):
        return self._prefix_len

def get_lookup(lookup_map):
    if "normalize" in lookup_map:
        return Embeddings(lookup_map["filename"])
    else:
        # A regular lookup table.
        return None
