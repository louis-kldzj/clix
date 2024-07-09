#NOTE: This is a functional approach. I'm going to implement a more oo approach to see how it reads / feels

import std/strformat
import std/dirs
import std/paths
import std/files

type
  RepoItemType = enum
    RootDirectory
    CommandDirectory,
    CommandScript,
    ConfigDirectory,
    ConfigFile,
    ResourceDirectory,
    ResourceFile

type
  BaseItem = object
    objType: RepoItemType
    path: Path
  RepoItem = object
    base: BaseItem
    children: seq[RepoItem]


proc constructItem(objType: RepoItemType, path: Path, children: seq[
    RepoItem]): RepoItem =
  result = RepoItem(
    base: BaseItem(
      objType: objType,
      path: path
    ),
    children: children
  )


proc toBase(path: Path): BaseItem =
  var itemType: RepoItemType
  if fileExists path:
    case path.parentDir().lastPathPart().string:
      of ".config":
        itemType = ConfigFile
      of ".resources":
        itemType = ResourceFile
      else:
        itemType = CommandScript
  else:
    case path.lastPathPart().string:
      of ".config":
        itemType = ConfigDirectory
      of ".resources":
        itemType = ResourceDirectory
      else:
        itemType = CommandDirectory
  result = BaseItem(objType: itemType, path: path)


## Repository


type
  Repository = object
    root: RepoItem


proc createRepoItem(path: Path): RepoItem =
  let baseItem = path.toBase()
  var children: seq[RepoItem] = @[]
  for child in walkDir path:
    children.add(createRepoItem(child.path))

  result = RepoItem(base: baseItem, children: children)


proc loadRepository*(path: string, relative: bool = true): Repository =
  let rootPath: Path = Path(path).absolutePath()
  var children: seq[RepoItem] = @[]

  for child in walkDir rootPath:
    children.add(createRepoItem(child.path))

  let root: RepoItem = constructItem(RepoItemType.RootDirectory, rootPath, children)

  result = Repository(root: root)
