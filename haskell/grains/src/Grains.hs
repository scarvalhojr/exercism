module Grains (square, total) where

import Data.Maybe (mapMaybe)

square :: Integer -> Maybe Integer
square n
    | n >= 1 && n <= 64 = Just (2 ^ (n - 1))
    | otherwise         = Nothing

total :: Integer
total = 2 ^ 64 - 1

-- If you really wanna use square...
-- total = sum $ mapMaybe square [1..64]
