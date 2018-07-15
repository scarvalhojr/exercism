module IsbnVerifier (isbn) where

import Data.Char (isDigit, digitToInt)

isbn :: String -> Bool
isbn xs
  | length digits /= 10                = False
  | any (not . isDigit) (init digits)  = False
  | lastd == 'X' || isDigit lastd      = validateISBN nums
  | otherwise                          = False
  where digits = filter (/= '-') xs
        lastd  = last xs
        nums   = map (\d -> if d == 'X' then 10 else digitToInt d) digits

validateISBN :: [Int] -> Bool
validateISBN xs = sumprod `mod` 11 == 0
  where sumprod = sum $ zipWith (*) [10,9..1] xs
