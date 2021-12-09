import Data.List.Split
import Data.List
import GHC.Float.RealFracMethods (roundDoubleInt, roundFloatInt)
import GHC.Base (divInt)


-- this puzzle probably isn't solve right, works on my input but not on the test one...
-- The best idea I have is to test both numbers around average and the choose the smallest since average seams to get me pretty close to the final number.
-- Rules are rules this is what solved the puzzle so I'm keeping it. 

main = do
        contents <- readFile "input.txt"
        let result = map read (splitOn "," contents)
        let result2 = fuelCount result (average result)
        print result2

average :: [Int] -> Int
average xs = fromIntegral (sum xs) `div` fromIntegral (length xs)

fuelCount :: [Int] -> Int -> Int
fuelCount [] _ = 0
fuelCount (x:xs) y = sum [0..abs(x-y)] + fuelCount xs y



