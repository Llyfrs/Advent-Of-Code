import Text.Read (Lexeme(String))
main = do  
        contents <- readFile "input.txt"
        print . count . map readInt . words $ contents
-- alternately, main = print . map readInt . words =<< readFile "test.txt"

readInt :: String -> Int
readInt = read

count [] = 0
count [x] = 0
count (x:y:xs) | x < y = 1 + count (y:xs) 
               | otherwise = count (y:xs) 
