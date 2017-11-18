module ETL (transform) where

import Data.Tuple (swap)
import Data.Char (toLower)
import Data.Map (Map, fromList, toList)

transform :: Map a String -> Map Char a
transform = fromList . concatMap transf . toList
    where transf (k, ls)  = zipWith ((,) . toLower) ls (repeat k)
