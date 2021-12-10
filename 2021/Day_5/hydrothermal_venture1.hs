import Data.List.Split
import Data.List

main = do
        contents <- readFile "input.txt"
        let result = length $ removeDuplicates (removeNoneDuplicates (sort (pointsWithVents . getCords $ lines contents)))
        print result

type Vents = [((Int,Int),(Int,Int))]

getCords :: [String] -> Vents
getCords [] = []
getCords (i:input) | x1 == x2 || y1 == y2 = ((x1,y1),(x2,y2)) : getCords input
                   | otherwise = getCords input
                where 
                    list = splitOn "->" i
                    x1 = read .head $ splitOn "," (head list)
                    x2 = read .head $ splitOn "," (last list)
                    y1 = read .last $ splitOn "," (head list)
                    y2 = read .last $ splitOn "," (last list)

pointsWithVents :: Vents -> [(Int,Int)]
pointsWithVents [] = []
pointsWithVents ((start,end):xs) = [(x,y) | x <- range (fst start)  (fst end), y <- range (snd start)  (snd end)] ++ pointsWithVents xs

range x y | x >= y = [y..x]
          | otherwise = [x..y]

removeNoneDuplicates [] = []
removeNoneDuplicates [x] = []
removeNoneDuplicates (x:y:xs) | x == y = x:removeNoneDuplicates (y:xs)
                              | otherwise = removeNoneDuplicates (y:xs)
removeDuplicates [] = []
removeDuplicates [x] = [x]
removeDuplicates (x:y:xs) | x == y = removeDuplicates (x:xs)
                          | otherwise = x:removeDuplicates (y:xs)

