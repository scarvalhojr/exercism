module School (School, add, empty, grade, sorted) where

import qualified Data.List as L (insert)
import qualified Data.Map as M (Map, insertWith, empty, findWithDefault, toList)

type Name = String
type Grade = Int
type School = M.Map Grade [Name]

add :: Grade -> Name -> School -> School
add grade name = M.insertWith (L.insert . head) grade [name]

empty :: School
empty = M.empty

grade :: Grade -> School -> [Name]
grade = M.findWithDefault []

sorted :: School -> [(Grade, [Name])]
sorted = M.toList
