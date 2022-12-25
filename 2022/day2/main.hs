outcome1 'A' 'X' = 4
outcome1 'A' 'Y' = 8
outcome1 'A' 'Z' = 3
outcome1 'B' 'X' = 1
outcome1 'B' 'Y' = 5
outcome1 'B' 'Z' = 9
outcome1 'C' 'X' = 7
outcome1 'C' 'Y' = 2
outcome1 'C' 'Z' = 6

part1 xs = sum $ map (\[x, y] -> outcome1 (head x) (head y)) xs

outcome2 'A' 'X' = 3
outcome2 'A' 'Y' = 4 
outcome2 'A' 'Z' = 8
outcome2 'B' 'X' = 1
outcome2 'B' 'Y' = 5 
outcome2 'B' 'Z' = 9
outcome2 'C' 'X' = 2 
outcome2 'C' 'Y' = 6 
outcome2 'C' 'Z' = 7 

part2 xs = sum $ map (\[x, y] -> outcome2 (head x) (head y)) xs

main = do 
    input <- getContents
    putStrLn $ show (part1 (map words (lines input)))
    putStrLn $ show (part2 (map words (lines input)))
