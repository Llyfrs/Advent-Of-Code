import Data.List.Split


-- Works for 80 but it isn't really optimized
-- So while the second solution is, this was the one I used to solve the part one of the problem so I'm going to keep it here
main = do
        contents <- readFile "input.txt"
        let result = dayPass 80 $ map read (splitOn "," contents)
        print result

dayPass 0 fishes = length fishes
dayPass day fishes = dayPass (day -1) (concatMap reproduce fishes)

reproduce a | a == 0 = [6,8]
            | otherwise = [a-1]