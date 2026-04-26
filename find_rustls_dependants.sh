#!/bin/bash
grep -B 25 "\"rustls\"" Cargo.lock | grep "name ="
