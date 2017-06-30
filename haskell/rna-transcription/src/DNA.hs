module DNA (toRNA) where

toRNA :: String -> Maybe String
toRNA = traverse translate
    where translate c = case c of
            'A' -> Just 'U'
            'C' -> Just 'G'
            'G' -> Just 'C'
            'T' -> Just 'A'
            _   -> Nothing
