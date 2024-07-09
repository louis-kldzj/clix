
import repo
import std/files
import std/paths


type
  Command = object
    name: string
    file: File

proc constructCommand(path: Path): Command =
  let pathAsString = path.lastPathPart.string
  let repository = loadRepository(pathAsString)
  let root = repository.root

