module SumOfMultiples (sumOfMultiples) where

sumOfMultiples :: [Integer] -> Integer -> Integer
sumOfMultiples factors limit = sum $ filter (`isFactor` factors) [1..limit - 1]
    where isFactor x = any ((== 0) . mod x)
