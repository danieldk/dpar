from functools import reduce

import tensorflow as tf
from tensorflow.contrib.layers import batch_norm
from enum import Enum

class Phase(Enum):
    train = 1
    validation = 2
    predict = 3

class Layer(Enum):
    token = 1
    tag = 2
    deprel = 3
    feature = 4
    char = 5

class ParseModel:
    def __init__(
            self,
            config,
            inputs,
            dataset,
            lookups,
            phase=Phase.train):
        batch_size = config.batch_size
        if phase in (Phase.train, Phase.validation):
            self._targets = tf.placeholder(tf.int32, config.batch_size, "targets")

        self._tokens = tf.placeholder(tf.int32, [batch_size, dataset.token_layer_size()],
                "tokens")
        self._tags = tf.placeholder(tf.int32, [batch_size, dataset.tag_layer_size()], "tags")
        self._deprels = tf.placeholder(tf.int32, [batch_size, dataset.deprel_layer_size()], "deprels")
        self._features = tf.placeholder(tf.int32, [batch_size, dataset.layer_size("features")], "features")
        self._chars = tf.placeholder(tf.int32, [batch_size, dataset.char_layer_size()], "chars")

        # For tokens, tags, and characters, we use pre-trained embeddings.
        # Todo: make this more flexible.
        self._token_embeds = tf.placeholder(tf.float32, lookups[Layer.token].data.shape, "token_embeds")
        self._tag_embeds = tf.placeholder(tf.float32, lookups[Layer.tag].data.shape, "tag_embeds")
        self._char_embeds = tf.placeholder(tf.float32, lookups[Layer.char].data.shape, "char_embeds")

        if phase == Phase.predict:
            batch_size = tf.shape(self._tokens)[0]

        token_input = tf.nn.embedding_lookup(self._token_embeds, self._tokens)
        token_input = tf.reshape(token_input, [batch_size, self._tokens.shape[1] * self._token_embeds.shape[1]])

        tag_input = tf.nn.embedding_lookup(self._tag_embeds, self._tags)
        tag_input = tf.reshape(tag_input, [batch_size, self._tags.shape[1] * self._tag_embeds.shape[1]])

        char_input = tf.nn.embedding_lookup(self._char_embeds, self._chars)
        char_input = tf.reshape(char_input, [batch_size,self._chars.shape[1] * self._char_embeds.shape[1]])
        with tf.variable_scope("char_norm"):
            char_input = tf.layers.batch_normalization(char_input, scale = True, momentum=0.80, training = phase == Phase.train, reuse = phase != Phase.train, fused=True)

        # The integer-encoded characters consist of n suffix/prefix pairs. Find n.
        n_morph_tokens = int(self._chars.shape[1]) // (inputs.prefix_len + inputs.suffix_len)
        char_input = tf.split(char_input, n_morph_tokens, 1)

        morph_w = tf.get_variable("morph_w", [char_input[0].get_shape()[1], config.morph_hidden_size])
        morph_b = tf.get_variable("morph_b", [config.morph_hidden_size])

        morph = list(map(lambda i: tf.nn.relu(tf.matmul(i, morph_w) + morph_b), char_input))

        morph = tf.concat(morph, 1)

        # For dependency relations, we train a separate layer, which could be seen as an
        # embeddings layer.
        with tf.device("/cpu:0"):
            deprel_embeds = tf.get_variable("deprel_embed", [dataset.max_deprel() + 1, config.deprel_embed_size])

        deprel_input = tf.nn.embedding_lookup(deprel_embeds, self._deprels)
        deprel_input = tf.reshape(deprel_input, [batch_size, self._deprels.shape[1] * deprel_embeds.shape[1]]) 

        # Features are converted to a one-hot representation.
        features = tf.one_hot(self._features, dataset.max_feature() + 1, axis = -1)
        features = tf.contrib.layers.flatten(features)

        inputs = tf.concat([token_input, tag_input, deprel_input, features, morph], 1, name="concat_inputs")
        with tf.variable_scope("input_norm"):
            inputs = tf.layers.batch_normalization(inputs, scale = True, momentum=0.98, training = phase == Phase.train, reuse = phase != Phase.train, fused=True)

        if config.keep_prob_input < 1 and phase == Phase.train:
            inputs = tf.nn.dropout(inputs, config.keep_prob_input)

        hidden_w = tf.get_variable("hidden_w", [inputs.get_shape()[1], config.hidden_size])
        hidden_b = tf.get_variable("hidden_b", [config.hidden_size])
        hidden = tf.matmul(inputs, hidden_w) + hidden_b
        hidden = tf.nn.relu(hidden)

        with tf.variable_scope("hidden_norm"):
            hidden = tf.layers.batch_normalization(hidden, scale = True, momentum = 0.97, training = phase == Phase.train, reuse = phase != Phase.train, fused=True)

        if config.keep_prob < 1 and phase == Phase.train:
            hidden = tf.nn.dropout(hidden, config.keep_prob)

        output_w = tf.get_variable("output_w", [config.hidden_size, config.num_outputs])
        output_b = tf.get_variable("output_b", [config.num_outputs])
        logits = tf.add(tf.matmul(hidden, output_w), output_b, name = "logits")

        if phase in (Phase.train, Phase.validation):
            losses = tf.nn.sparse_softmax_cross_entropy_with_logits(
                logits = logits, labels = self._targets)
            self._loss = loss = tf.reduce_sum(losses)

        if phase in (Phase.validation, Phase.train):
            _, labels = tf.nn.top_k(logits)
            labels = tf.reshape(labels,[-1])
            correct = tf.equal(self._targets, labels)
            correct = tf.cast(correct, tf.float32)
            self._accuracy = tf.reduce_sum(correct) / batch_size
        if phase == Phase.train:
            self._lr = tf.Variable(0.0, trainable=False)
            update_ops = tf.get_collection(tf.GraphKeys.UPDATE_OPS)
            with tf.control_dependencies(update_ops):
                self._train_op = tf.train.AdagradOptimizer(self.lr).minimize(loss)

        if phase == Phase.predict:
            #_, labels = tf.nn.top_k(logits)
            #predicted = tf.reshape(labels, [-1], name="predicted")
            predictions = tf.reshape(logits, [-1], name="logits")

    @property
    def accuracy(self):
        return self._accuracy

    @property
    def correct(self):
        return self._correct

    def assign_lr(self, session, lr_value):
        session.run(tf.assign(self.lr, lr_value))

    @property
    def char_embeds(self):
        return self._char_embeds

    @property
    def chars(self):
        return self._chars

    @property
    def deprels(self):
        return self._deprels

    @property
    def features(self):
        return self._features

    @property
    def tags(self):
        return self._tags

    @property
    def tokens(self):
        return self._tokens

    @property
    def tag_embeds(self):
        return self._tag_embeds

    @property
    def token_embeds(self):
        return self._token_embeds

    @property
    def loss(self):
        return self._loss

    @property
    def lr(self):
        return self._lr

    @property
    def train_op(self):
        return self._train_op

    @property
    def targets(self):
        return self._targets
