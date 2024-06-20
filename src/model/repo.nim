import std/strformat
import std/os

type
  RepoObjectType = enum
    RootDirectory
    CommandDirectory,
    CommandScript,
    ComfigDirectory,
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


proc loadRepository(path: string): Repository =
  let root = RepoObject(
    obj_type: RootDirectory,
    path: path,
    children: @[]
  )
