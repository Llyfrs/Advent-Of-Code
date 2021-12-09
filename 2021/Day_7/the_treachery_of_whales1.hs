import Data.List.Split
import Data.List


main = do
        contents <- readFile "input.txt"
        let result = map read (splitOn "," contents)
        let result2 = fuelCount result (median result) 
        print result2


median :: [Int] -> Int
median xs = sort xs!!div (length xs) 2

fuelCount :: [Int] -> Int -> Int 
fuelCount [] _ = 0
fuelCount (x:xs) y = abs(x-y) + fuelCount xs y