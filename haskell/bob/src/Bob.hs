module Bob (responseFor) where

import Data.Char (isUpper, isLower, isSpace)

data Tone = Normal | Loud | Unknown

data Expression = Silence | Statement | Question

responseFor :: String -> String
responseFor xs = respond (readInput xs)

respond :: (Tone, Expression) -> String
respond (Loud, _)     = "Whoa, chill out!"
respond (_, Question) = "Sure."
respond (_, Silence)  = "Fine. Be that way!"
respond (_, _)        = "Whatever."

readInput :: String -> (Tone, Expression)
readInput = readNextChar (Unknown, Silence)

readNextChar :: (Tone, Expression) -> String -> (Tone, Expression)
readNextChar (t,e) []       = (t,e)
readNextChar (t,_) ('?':xs) = readNextChar (t, Question) xs
readNextChar (Unknown, Silence) (x:xs)
    | isSpace x             = readNextChar (Unknown, Silence) xs
    | isLower x             = readNextChar (Normal, Statement) xs
    | isUpper x             = readNextChar (Loud, Statement) xs
    | otherwise             = readNextChar (Unknown, Statement) xs
readNextChar (Unknown, exp) (x:xs)
    | isSpace x             = readNextChar (Unknown, exp) xs
    | isLower x             = readNextChar (Normal, Statement) xs
    | isUpper x             = readNextChar (Loud, Statement) xs
    | otherwise             = readNextChar (Unknown, Statement) xs
readNextChar (Loud, exp) (x:xs)
    | isSpace x             = readNextChar (Loud, exp) xs
    | isLower x             = readNextChar (Normal, Statement) xs
    | otherwise             = readNextChar (Loud, Statement) xs
readNextChar (Normal, exp) (x:xs)
    | isSpace x             = readNextChar (Normal, exp) xs
    | otherwise             = readNextChar (Normal, Statement) xs
