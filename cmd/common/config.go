package common

import (
	"io"

	"github.com/BurntSushi/toml"
)

type DParConfig struct {
	Parser Parser
}

type Parser struct {
	System         string
	Features       string
	Model          string
	Transitions    string
	HashKernelSize uint `toml:"hash_kernel_size"`
}

func defaultConfiguration() *DParConfig {
	return &DParConfig{
		Parser: Parser{
			System:         "stackproj",
			Features:       "parser.features",
			Model:          "parser.model",
			Transitions:    "parser.transitions",
			HashKernelSize: 300000,
		},
	}
}

func ParseConfig(reader io.Reader) (*DParConfig, error) {
	config := defaultConfiguration()
	if _, err := toml.DecodeReader(reader, config); err == nil {
		return config, nil
	} else {
		return config, err
	}
}
