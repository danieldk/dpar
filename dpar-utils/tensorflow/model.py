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
            shapes):
        batch_size = None

        # Are we training or not?
        self._is_training = tf.placeholder(tf.bool, [], "is_training")

        # Labels for training and validation.
        self._targets = tf.placeholder(tf.int32, batch_size, "targets")

        n_tokens = int(shapes['tokens'])
        self._tokens = tf.placeholder(tf.int32, [batch_size, n_tokens],
                                      "tokens")

        n_tags = int(shapes['tags'])
        self._tags = tf.placeholder(tf.int32, [batch_size, n_tags], "tags")

        n_deprels = int(shapes['deprels'])
        self._deprels = tf.placeholder(
            tf.int32, [batch_size, n_deprels], "deprels")

        n_features = int(shapes['features'])
        self._features = tf.placeholder(
            tf.int32, [batch_size, n_features], "features")

        n_chars = int(shapes['chars'])
        self._chars = tf.placeholder(tf.int32, [batch_size, n_chars], "chars")

        # For tokens, tags, and characters, we use pre-trained embeddings.
        # Todo: make this more flexible.
        self._token_embeds = tf.placeholder(
            tf.float32, [None, 50], "token_embeds")
        self._tag_embeds = tf.placeholder(tf.float32, [None, 50], "tag_embeds")
        self._char_embeds = tf.placeholder(
            tf.float32, [None, 50], "char_embeds")

        token_input = tf.nn.embedding_lookup(self._token_embeds, self._tokens)
        token_input = tf.reshape(
            token_input, [
                tf.shape(
                    self._tokens)[0], self._tokens.shape[1] * self._token_embeds.shape[1]])

        tag_input = tf.nn.embedding_lookup(self._tag_embeds, self._tags)
        tag_input = tf.reshape(
            tag_input, [
                tf.shape(
                    self._tags)[0], self._tags.shape[1] * self._tag_embeds.shape[1]])

        char_input = tf.nn.embedding_lookup(self._char_embeds, self._chars)
        char_input = tf.reshape(
            char_input, [
                tf.shape(
                    self._chars)[0], self._chars.shape[1] * self._char_embeds.shape[1]])
        with tf.variable_scope("char_norm"):
            char_input = tf.layers.batch_normalization(
                char_input, scale=True, momentum=0.80, training=self.is_training, fused=True)

        # The integer-encoded characters consist of n suffix/prefix pairs. Find
        # n.
        prefix_len = int(shapes["prefix_len"])
        suffix_len = int(shapes["suffix_len"])
        n_morph_tokens = int(self._chars.shape[1]) // (prefix_len + suffix_len)
        char_input = tf.split(char_input, n_morph_tokens, 1)

        morph_w = tf.get_variable(
            "morph_w", [
                char_input[0].get_shape()[1], config.morph_hidden_size])
        morph_b = tf.get_variable("morph_b", [config.morph_hidden_size])

        morph = list(map(lambda i: tf.nn.relu(
            tf.matmul(i, morph_w) + morph_b), char_input))
        morph = tf.concat(morph, 1)

        # For dependency relations, we train a separate layer, which could be seen as an
        # embeddings layer.
        n_deprel_embeds = int(shapes["deprel_embeds"])
        with tf.device("/cpu:0"):
            deprel_embeds = tf.get_variable(
                "deprel_embed", [
                    n_deprel_embeds, config.deprel_embed_size])

        deprel_input = tf.nn.embedding_lookup(deprel_embeds, self._deprels)
        deprel_input = tf.reshape(deprel_input, [tf.shape(self._deprels)[
            0], self._deprels.shape[1] * deprel_embeds.shape[1]])

        n_attachment_addrs = 2
        self._assoc_strengths = tf.placeholder(
            tf.float32, [batch_size, n_deprel_embeds * n_attachment_addrs], "assoc_strengths")

        # Features are converted to a one-hot representation.
        n_features = int(shapes["n_features"])
        features = tf.one_hot(self._features, n_features, axis=-1)
        features = tf.contrib.layers.flatten(features)

        inputs = tf.concat([token_input,
                            tag_input,
                            deprel_input,
                            features,
                            morph,
                            self._assoc_strengths],
                           1,
                           name="concat_inputs")
        with tf.variable_scope("input_norm"):
            inputs = tf.layers.batch_normalization(
                inputs, scale=True, momentum=0.98, training=self.is_training, fused=True)

        if config.keep_prob_input < 1:
            inputs = tf.contrib.layers.dropout(
                inputs,
                keep_prob=config.keep_prob_input,
                is_training=self.is_training)

        hidden_w = tf.get_variable(
            "hidden_w", [
                inputs.get_shape()[1], config.hidden_size])
        hidden_b = tf.get_variable("hidden_b", [config.hidden_size])
        hidden = tf.matmul(inputs, hidden_w) + hidden_b
        hidden = tf.nn.relu(hidden)

        with tf.variable_scope("hidden_norm"):
            hidden = tf.layers.batch_normalization(
                hidden, scale=True, momentum=0.97, training=self.is_training, fused=True)

        if config.keep_prob < 1:
            hidden = tf.contrib.layers.dropout(
                hidden, keep_prob=config.keep_prob, is_training=self.is_training)

        n_labels = int(shapes["n_labels"])
        output_w = tf.get_variable("output_w", [config.hidden_size, n_labels])
        output_b = tf.get_variable("output_b", [n_labels])
        logits = tf.add(tf.matmul(hidden, output_w), output_b, name="logits")

        losses = tf.nn.sparse_softmax_cross_entropy_with_logits(
            logits=logits, labels=self._targets)
        self._loss = loss = tf.reduce_sum(losses, name="loss")

        _, labels = tf.nn.top_k(logits)
        labels = tf.reshape(labels, [-1])
        correct = tf.equal(self._targets, labels)
        self._accuracy = tf.divide(
            tf.reduce_sum(
                tf.cast(
                    correct, tf.float32)), tf.cast(
                tf.shape(
                    self._targets)[0], tf.float32), name="accuracy")

        self._lr = tf.placeholder(tf.float32, [], "lr")
        update_ops = tf.get_collection(tf.GraphKeys.UPDATE_OPS)
        with tf.control_dependencies(update_ops):
            self._train_op = tf.train.AdagradOptimizer(
                self.lr).minimize(loss, name="train")

    @property
    def accuracy(self):
        return self._accuracy

    @property
    def correct(self):
        return self._correct

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
    def is_training(self):
        return self._is_training

    @property
    def features(self):
        return self._features

    @property
    def lr(self):
        return self._lr

    @property
    def assoc_strengths(self):
        return self._assoc_strengths

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
    def train_op(self):
        return self._train_op

    @property
    def targets(self):
        return self._targets
