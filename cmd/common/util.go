// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package common

import "log"

func ExitIfError(err error) {
	if err != nil {
		log.Fatal(err.Error())
	}
}
