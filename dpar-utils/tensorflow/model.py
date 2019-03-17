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

        self._embeds = tf.placeholder(tf.float32, [batch_size, shapes['embed_size']], "embeds")

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

        # Features are converted to a one-hot representation.
        n_features = int(shapes["n_features"])
        features = tf.one_hot(self._features, n_features, axis=-1)
        features = tf.contrib.layers.flatten(features)

        inputs = tf.concat([self.embeds,
                            deprel_input,
                            features],
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
    def deprels(self):
        return self._deprels

    @property
    def embeds(self):
        return self._embeds

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
