#!/bin/bash
grep -B 25 "\"ring\"" Cargo.lock | grep "name ="
