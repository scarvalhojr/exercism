module RunLength (decode, encode) where

import Data.Char (isDigit)
import Data.List (group)

decode :: String -> String
decode = decode' 0
    where decode' _ []      = ""
          decode' times (x:xs)
            | isDigit x     = decode' (10 * times + (read [x] :: Int)) xs
            | otherwise     = replicate (max 1 times) x ++ decode' 0 xs

encode :: String -> String
encode = concatMap encodeGroup . group
    where encodeGroup [x]   = [x]
          encodeGroup g     = show (length g) ++ [head g]
