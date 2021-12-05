main = do  
        contents <- readFile "input.txt"
        print . count . map readInt . words $ contents

readInt :: String -> Int
readInt = read

count :: [Int] -> Int
count [] = 0
count xs | sumThree xs < sumThree (tail xs) = 1 + count (tail xs)
         | otherwise = count $ tail xs
         
sumThree :: [Int] -> Int
sumThree [] = 0
sumThree [x] = 0
sumThree [x,y] = 0 
sumThree (x:y:z:_) = x + y +z 