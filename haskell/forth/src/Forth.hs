{-# LANGUAGE OverloadedStrings #-}

module Forth
  ( ForthError(..)
  , ForthState
  , evalText
  , toList
  , empty
  ) where

import           Data.Text            (Text)
import           Data.Char            (isDigit)
import qualified Data.Text      as T  (words, unpack, toLower, all)
import qualified Data.Map       as M  (Map, empty, insert, findWithDefault,
                                       member)

data ForthError
     = DivisionByZero
     | StackUnderflow
     | InvalidWord
     | UnknownWord Text
     deriving (Show, Eq)

data ForthState
     = ForthState { getStack :: [Int]
                  , getDefWords :: M.Map Text [Text] }

data ForthExpr
     = Value Int
     | ExecOper Text
     | DefWord Text [Text]

empty :: ForthState
empty = ForthState [] M.empty

evalText :: Text -> ForthState -> Either ForthError ForthState
evalText = evalTokens . T.words

evalTokens :: [Text] -> ForthState -> Either ForthError ForthState
evalTokens [] state = Right state
evalTokens tokens state =
  case parseExpr tokens of
    Right (rest, expr) -> evalExpr expr state >>= evalTokens rest
    Left err           -> Left err

parseExpr :: [Text] -> Either ForthError ([Text], ForthExpr)
parseExpr (":":name:ts)
  | isNumber name  = Left InvalidWord
  | null rest      = Right ([], DefWord name tokens)
  | otherwise      = Right (tail rest, DefWord name tokens)
  where (tokens, rest) = break (== ";") ts
parseExpr (t:ts)
  | isNumber t    = Right (ts, Value n)
  | otherwise     = Right (ts, ExecOper t)
  where n = read (T.unpack t)

isNumber :: Text -> Bool
isNumber = T.all isDigit

evalExpr :: ForthExpr -> ForthState -> Either ForthError ForthState
evalExpr (Value n)             = pushValue n
evalExpr (ExecOper w)          = execOper w
evalExpr (DefWord name tokens) = defWord name tokens

pushValue :: Int -> ForthState -> Either ForthError ForthState
pushValue n (ForthState stack d) = Right $ ForthState (n:stack) d

execOper :: Text -> ForthState -> Either ForthError ForthState
execOper w state
  | not (null tokens)  = evalTokens tokens state
  | w == "+"           = arithOper (+) state
  | w == "-"           = arithOper (-) state
  | w == "*"           = arithOper (*) state
  | w == "/"           = divOper state
  | lowerW == "dup"    = dupOper state
  | lowerW == "drop"   = dropOper state
  | lowerW == "swap"   = swapOper state
  | lowerW == "over"   = overOper state
  | otherwise          = Left $ UnknownWord w
  where tokens = M.findWithDefault [] lowerW (getDefWords state)
        lowerW = T.toLower w

arithOper :: (Int -> Int -> Int) -> ForthState -> Either ForthError ForthState
arithOper op (ForthState (x1:x2:xs) d) = Right $ ForthState (op x2 x1:xs) d
arithOper _  _                         = Left StackUnderflow

divOper :: ForthState -> Either ForthError ForthState
divOper (ForthState (0:x2:xs)  _) = Left DivisionByZero
divOper (ForthState (x1:x2:xs) d) = Right $ ForthState (x2 `div` x1:xs) d
divOper _                         = Left StackUnderflow

dupOper :: ForthState -> Either ForthError ForthState
dupOper (ForthState (x:xs) d) = Right $ ForthState (x:x:xs) d
dupOper _                     = Left StackUnderflow

dropOper :: ForthState -> Either ForthError ForthState
dropOper (ForthState (x:xs) d) = Right $ ForthState xs d
dropOper _                     = Left StackUnderflow

swapOper :: ForthState -> Either ForthError ForthState
swapOper (ForthState (x1:x2:xs) d) = Right $ ForthState (x2:x1:xs) d
swapOper _                         = Left StackUnderflow

overOper :: ForthState -> Either ForthError ForthState
overOper (ForthState (x1:x2:xs) d) = Right $ ForthState (x2:x1:x2:xs) d
overOper _                         = Left StackUnderflow

defWord :: Text -> [Text] -> ForthState -> Either ForthError ForthState
defWord name tokens (ForthState s d) = Right $ ForthState s d'
  where d' = M.insert (T.toLower name) tokens d

toList :: ForthState -> [Int]
toList = reverse . getStack
