#!/usr/bin/env python

import math
import numpy as np
import sys
import tensorflow as tf
import toml

from config import DefaultConfig
from data import DataSet, Embeddings
from inputs import Inputs, get_lookup
from model import Layer, Phase, ParseModel
from tensorflow.contrib.session_bundle import exporter

def usage():
    print("Usage: %s config train_data graph_ouy" % sys.argv[0])
    sys.exit(1)

def main(unused_args):
    if len(unused_args) != 4:
        usage()

    config = DefaultConfig()
    predict_config = DefaultConfig()
    predict_config.batch_size = None

    with open(unused_args[1]) as conffile:
        parser_config = toml.loads(conffile.read())

    inputs = Inputs(parser_config["parser"]["inputs"])

    with DataSet(unused_args[2]) as train_ds:
        config.num_outputs = train_ds.max_label() + 1
        predict_config.num_outputs = train_ds.max_label() + 1

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

        dump_model(config=config,
                inputs=inputs,
                predict_config=predict_config,
                train_dataset = train_ds,
                lookups = lookups,
                graph_out = unused_args[3])

def dump_model(
        config,
        inputs,
        predict_config,
        train_dataset,
        lookups,
        graph_out):
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

        with tf.name_scope("prediction"):
            with tf.variable_scope("model", reuse=True, initializer=initializer):
                mpredict = ParseModel(
                    config = predict_config,
                    inputs = inputs,
                    phase=Phase.predict,
                    dataset=train_dataset,
                    lookups=lookups)

        init_op = tf.initialize_all_variables()
        session.run(init_op)

        tf.train.write_graph(
            session.graph_def,
            './',
            graph_out,
            as_text=True)

if __name__ == "__main__":
    tf.app.run()
