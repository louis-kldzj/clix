import std/strformat
import std/os
import model/repo

let source_dir = "/Users/louis/code/exCLInce/"

echo fmt"source dir: {source_dir}"

let loaded_repo = loadRepository source_dir

echo fmt "ldedrpo: {loaded_repo}"
