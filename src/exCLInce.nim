import std/strformat
import std/os
import model/repo

let source_dir = "/home/locuris/code/clix-exCLInce/"

echo fmt"source dir: {source_dir}"

for match in walkDir source_dir:
  case match.kind:
  of pcDir:
    echo fmt"dir: {match.path}"
  of pcFile:
    echo fmt"file: {match}"
  else: echo fmt"link: {match}"

