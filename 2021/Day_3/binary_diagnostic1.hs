
main = do
        contents <- readFile "input.txt"
        let result = power $ lines contents
        print result


power xs = bitToInt (length (head xs) -1) (diagnostic xs) * bitToInt (length (head xs) -1) (map revers $ diagnostic xs)

diagnostic :: [String] -> [Int]
diagnostic [] = []
diagnostic xs | head xs == "" = []
              | result > size = 1: diagnostic rest
              | otherwise = 0 : diagnostic rest
              where
                      result = sum $ map (read . take 1) xs
                      size = length xs `div` 2
                      rest = map (drop 1) xs

-- reversed the bit order
revers x = if x == 0 then 1 else 0
-- converts bits to full integer
bitToInt ::  Int -> [Int] -> Int
bitToInt _ [] = 0
bitToInt r (x:xs) = (x * 2^r) + bitToInt (r-1) xs
