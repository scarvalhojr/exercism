module Pangram (isPangram) where

import Data.List (sort, group)
import Data.Char (isLetter, toLower)


isPangram :: String -> Bool
isPangram text = length (letters text) == 26
    where
        letters = rmdups . map toLower . filter isLetter
        rmdups = map head . group . sort
