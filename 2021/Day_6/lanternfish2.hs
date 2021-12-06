import Data.List.Split
import Data.List
import Control.Concurrent.Async (waitEitherCancel)
main = do
        contents <- readFile "input.txt"
        let result = dayPass 256 (compress $ map read (splitOn "," contents))
        print result

dayPass 0 fishes = lengthC fishes
dayPass day fishes = dayPass (day -1) (recompress $ concatMap reproduce fishes)

lengthC [] = 0
lengthC ((_,number):xs) = number + lengthC xs
reproduce (age,number) | age == 0 = [(6,number),(8,number)]
                       | otherwise = [(age-1,number)]

compress :: [Int] -> [(Int,Int)]
compress list = compress' $ sort list
                where
                compress' :: [Int] -> [(Int,Int)]
                compress' [] = []
                compress' list = (head result, length result) : compress' (drop (length result) list)
                        where
                        result = filter (==head list) list

recompress :: [(Int,Int)] -> [(Int,Int)]
recompress list = recompress' $ sort list
                  where
                  recompress' [] = []
                  recompress' list = (fst $ head result, lengthC result) : recompress' (drop (length result) list)
                          where
                          result = filter (\(x,y) -> x == fst (head list)) list