import std/strformat
import std/os

type
  RepoObjectType = enum
    RootDirectory
    CommandDirectory,
    CommandScript,
    ConfigDirectory,
    ConfigFile,
    ResourceDirectory,
    ResourceFile

type
  RepoObject = object
    obj_type: RepoObjectType
    path: string
    children: seq[RepoObject]

type
  Repository = object
    root: RepoObject

proc readDirectoryLevel(path: string, dir_type: RepoObjectType): seq[RepoObject]

proc loadRepository*(path: string): Repository =
  let root = RepoObject(
    obj_type: RootDirectory,
    path: path,
    children: path.readDirectoryLevel CommandDirectory
  )

  result = Repository(root: root)


proc createFileObject(path: string, parentType: RepoObjectType): RepoObject =
  var file_type: RepoObjectType
  case parentType:
    of ConfigDirectory:
      file_type = ConfigFile
    of ResourceDirectory:
      file_type = ResourceFile
    else:
      file_type = CommandScript

  result = RepoObject(
    obj_type: file_type,
    path: path,
    children: @[]
  )


proc createDirectoryObject(path: string): RepoObject =
  var dir_type: RepoObjectType
  case path:
    of ".config":
      dir_type = ConfigDirectory
    of ".resources":
      dir_type = ResourceDirectory
    else:
      dir_type = CommandDirectory

  result = RepoObject(
    obj_type: dir_type,
    path: path,
    children: path.readDirectoryLevel dir_type
  )

proc readDirectoryLevel(path: string, dir_type: RepoObjectType): seq[RepoObject] =
  var children: seq[RepoObject] = @[]
  for obj in walkDir path:
    case obj.kind:
      of pcDir:
        children.add(createDirectoryObject obj.path)
      of pcFile:
        children.add(obj.path.createFileObject dir_type)

      else:
        echo fmt"unhandled directory object type: {obj.path}"

  result = children


