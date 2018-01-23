#!/usr/bin/env python

import math
import sys
import numpy as np
import tensorflow as tf
import toml

from config import DefaultConfig
from data import DataSet, Embeddings
from inputs import Inputs, get_lookup
from model import Layer, Phase, ParseModel
from keras.utils import generic_utils

def usage():
    print("Usage: %s train CONFIG TRAIN.HDF5 VALIDATION.HDF5" % sys.argv[0])
    sys.exit(1)

def main(unused_args):
    if len(unused_args) != 4:
        usage()

    config = DefaultConfig()

    with open(unused_args[1]) as conffile:
        parser_config = toml.loads(conffile.read())

    inputs = Inputs(parser_config["parser"]["inputs"])

    with DataSet(unused_args[2]) as train_ds,\
            DataSet(unused_args[3]) as valid_ds:
        config.num_outputs = train_ds.max_label() + 1

        print("Batch size: %d" % config.batch_size)
        print("Token size: %d" % train_ds.token_layer_size())
        print("Tag size: %d" % train_ds.tag_layer_size())
        print("Deprel size: %d" % train_ds.deprel_layer_size())
        print("Features size: %d" % train_ds.layer_size("features"))
        print("Char size: %d (%d tokens)" % (train_ds.char_layer_size(),
            train_ds.char_layer_size() / 8))
        print("Outputs: %d" % config.num_outputs)

        lookups_conf = parser_config["lookups"]
        token_lookup = get_lookup(lookups_conf["word"])
        tag_lookup = get_lookup(lookups_conf["tag"])
        deprel_lookup = get_lookup(lookups_conf["deprel"])
        feature_lookup = get_lookup(lookups_conf["feature"])
        char_lookup = get_lookup(lookups_conf["chars"])

        lookups = {
                Layer.token: token_lookup,
                Layer.tag: tag_lookup,
                Layer.deprel: deprel_lookup,
                Layer.feature: feature_lookup,
                Layer.char: char_lookup,
        }

        train_model(config=config,
                inputs = inputs,
                train_dataset = train_ds,
                valid_dataset = valid_ds,
                lookups = lookups)

def run_epoch(session, m, config, dataset, eval_op, lookups, verbose=False, accuracy=False):
    """Runs the model on the given data."""
    losses = 0.0
    accs = 0.0
    batch = 0

    n_batches = dataset.n_batches

    pb = generic_utils.Progbar(n_batches)

    for step in range(n_batches):
        token = dataset.batch_layer(step, 'tokens')
        tag = dataset.batch_layer(step, 'tags')
        deprel = dataset.batch_layer(step, 'deprels')
        features = dataset.batch_layer(step, 'features')
        char = dataset.batch_layer(step, 'chars')
        y = dataset.batch_layer(step, 'labels')

        # XXX - Debug
        accuracy = True

        if accuracy:
            cost, acc, _ = session.run([m.loss, m.accuracy, eval_op],
                    { m.tokens: token,
                      m.tags: tag,
                      m.deprels: deprel,
                      m.features: features,
                      m.chars: char,
                      m.token_embeds: lookups[Layer.token].data,
                      m.tag_embeds: lookups[Layer.tag].data,
                      m.char_embeds: lookups[Layer.char].data,
                      m.targets: y})
            accs += acc
        else:
            cost, _ = session.run([m.loss, eval_op],
                    { m.tokens: token,
                      m.tags: tag,
                      m.deprels: deprel,
                      m.features: features,
                      m.chars: char,
                      m.token_embeds: np.reshape(lookups[Layer.token].data, (-1)),
                      m.tag_embeds: np.reshape(lookups[Layer.tag].data, (-1)),
                      m.char_embeds: lookups[Layer.char].data,
                      m.targets: y})

        losses += cost
        batch += 1

        pb.update(batch)

    accs /= n_batches
    losses /= n_batches

    return losses, accs

def train_model(
        config,
        inputs,
        train_dataset,
        valid_dataset,
        lookups):
    gpuopts = tf.GPUOptions(per_process_gpu_memory_fraction=0.3)
    tfconfig = tf.ConfigProto(gpu_options=gpuopts)

    with tf.Graph().as_default(), tf.Session(config=tfconfig) as session:
        initializer = tf.random_uniform_initializer(-config.init_scale,
                                                    config.init_scale)


        with tf.name_scope("training"):
            with tf.variable_scope("model", reuse=None, initializer=initializer):
                m = ParseModel(
                    config = config,
                    inputs = inputs,
                    phase=Phase.train,
                    dataset=train_dataset,
                    lookups=lookups)

        with tf.name_scope("validation"):
            with tf.variable_scope("model", reuse=True, initializer=initializer):
                mvalid = ParseModel(
                    config=config,
                    inputs = inputs,
                    phase=Phase.validation,
                    # The data set is only used to build shapes in the model
                    # constructor. We pass the training data set, because it
                    # is guiding in e.g. the number of different dependency
                    # relations, features, etc.
                    dataset=train_dataset,
                    lookups=lookups)

        init_op = tf.initialize_all_variables()
        session.run(init_op)

        saver = tf.train.Saver(max_to_keep=0)

        for i in range(config.max_epoch):
            if i < 10:
                lr = 0.05
            else:
                lr = 0.05 * (1 + 0.2 * (i - 9)) ** -2
            #lr = 0.05
            m.assign_lr(session, lr)

            print("Epoch: %d Learning rate: %.3f" % (i + 1, session.run(m.lr)))
            train_loss, accs = run_epoch(
                    session=session,
                    config=config,
                    m=m,
                    dataset=train_dataset,
                    eval_op=m.train_op,
                    lookups=lookups,
                    verbose=True,
                    accuracy=False)
            print("Epoch: %d Train Loss: %.3f" % (i + 1, train_loss))
            print("Epoch: %d Train accuracy: %.3f" % (i + 1, accs))

            valid_loss, accs = run_epoch(
                session=session,
                config=config,
                m=mvalid,
                dataset=valid_dataset,
                eval_op=tf.no_op(),
                lookups=lookups,
                accuracy=True)
            print("Epoch: %d Validation loss: %.3f" % (i + 1, valid_loss))
            print("Epoch: %d Validation accuracy: %.3f" % (i + 1, accs))

            save_path = saver.save(session, "./model", global_step=i + 1)

if __name__ == "__main__":
    tf.app.run()
