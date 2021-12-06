
main = do  
        contents <- readFile "input.txt"
        let result = dive (lines contents) 0 0 0
        putStrLn $ show result


dive :: [String] -> Int -> Int -> Int -> Int
dive [] x y _ = x * y 
dive (line:lines) x y a | action == "forward" = dive lines (x+read value) (y+(a*read value)) a
                        | action == "up" = dive lines x y (a-read value) 
                        | action == "down" = dive lines x y (a+read value) 
                        | otherwise = dive lines x y a
                 where 
                   action = (words line)!!0
                   value =  (words line)!!1
