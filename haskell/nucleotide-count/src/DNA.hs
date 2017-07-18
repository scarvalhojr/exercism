module DNA (nucleotideCounts) where

import Data.Map (Map, fromList, adjust)

nucleotides :: String
nucleotides = "ACGT"

nucleotideCounts :: String -> Either String (Map Char Int)
nucleotideCounts []     = Right $ fromList (zip nucleotides (repeat 0))
nucleotideCounts (x:xs)
    | x `elem` nucleotides  = adjust succ x <$> nucleotideCounts xs
    | otherwise             = Left "Invalid DNA"
