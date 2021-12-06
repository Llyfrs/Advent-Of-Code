main = do  
        contents <- readFile "input.txt"
        let result = count $ map read (words contents)
        print result

count :: [Int] -> Int
count [] = 0
count xs | sumThree xs < sumThree (tail xs) = 1 + count (tail xs)
         | otherwise = count $ tail xs
         
sumThree :: [Int] -> Int
sumThree [] = 0
sumThree [x] = 0
sumThree [x,y] = 0 
sumThree (x:y:z:_) = x + y +z 