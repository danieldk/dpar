from gensim.models import KeyedVectors
import h5py
import math
import numpy as np

class DataSet:
    def __init__(self, filename):
        self._data = h5py.File(filename, 'r', driver='core')
        self._n_batches = self._data['batches'][0]

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        if self._data != None:
            self._data.close()

    def char_layer_size(self):
        return self.data["batch0-chars"].shape[1]

    def get_data(self):
        return self._data

    def get_n_batches(self):
        return self._n_batches

    def batch_layer(self, batch, layer):
        return self._data["batch%d-%s" % (batch, layer)]

    def layer_size(self, layer):
        return self._data["batch0-%s" % layer].shape[1]

    def deprel_layer_size(self):
        return self.data["batch0-deprels"].shape[1]

    def max_deprel(self):
        return self._max_value("deprels")

    def max_label(self):
        return self._max_value("labels")

    def max_feature(self):
        return self._max_value("features")

    def _max_value(self, layer_str):
        max_value = 0

        for batch in range(self._n_batches):
            max_value = max(np.max(self._data["batch%d-%s" % (batch, layer_str)]), max_value)

        return max_value


    def tag_layer_size(self):
        return self.data["batch0-tags"].shape[1]

    def token_layer_size(self):
        return self.data["batch0-tokens"].shape[1]


    data = property(get_data)
    n_batches = property(get_n_batches)

class Embeddings:
    def __init__(self, filename, normalize = True):
        embeds = KeyedVectors.load_word2vec_format(filename, binary=True)
        # We only care about the actual embeddings.
        self._data = embeds.syn0

        if normalize:
            self.normalize()

    def get_data(self):
        return self._data

    def normalize(self):
        # Note: gensim does not check whether the L2 norm is 0.
        for i in range(self._data.shape[0]):
            l2norm = math.sqrt((self._data[i, :] ** 2).sum(-1))
            if l2norm != 0.:
                self._data[i, :] /= l2norm


    data = property(get_data)
