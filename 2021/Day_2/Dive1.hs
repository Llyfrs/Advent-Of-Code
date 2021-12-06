
main = do  
        contents <- readFile "input.txt"
        let result = dive (lines contents) 0 0
        putStrLn $ show result


dive :: [String] -> Int -> Int -> Int
dive [] x y = x * y 
dive (line:lines) x y | action == "forward" = dive lines (x+read value) y
                      | action == "up" = dive lines x (y-read value) 
                      | action == "down" = dive lines x (y+read value)
                      | otherwise = dive lines x y 
                 where 
                   action = (words line)!!0
                   value =  (words line)!!1
