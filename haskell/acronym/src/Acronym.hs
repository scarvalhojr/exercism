module Acronym (abbreviate) where

import Data.Char (isSpace, isLetter, isLower, isUpper, toUpper)

abbreviate :: String -> String
abbreviate = concatMap initials . split

split :: String -> [String]
split [] = []
split (x:xs)
    | isSeparator x = split xs
    | otherwise     = word : split rest
    where (word, rest) = break isSeparator (x:xs)

isSeparator :: Char -> Bool
isSeparator c = isSpace c || c == '-'

initials :: String -> String
initials "" = ""
initials w
    | all isUpper letters   = [head w]
    | all isLower letters   = [toUpper (head w)]
    | otherwise             = filter isUpper w
    where letters = filter isLetter w
