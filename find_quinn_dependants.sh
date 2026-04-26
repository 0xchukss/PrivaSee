#!/bin/bash
grep -B 20 "\"quinn\"," Cargo.lock | grep "name ="
