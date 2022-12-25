import Data.List

calories xs = 
    let first = sum $ map read (takeWhile (/= "") xs)
        rest = dropWhile (/= "") xs
    in  if null rest then [first] else
        first:(calories (tail rest))

part1 xs = maximum $ calories xs

part2 xs = sum . take 3 . sortBy (flip compare) $ calories xs

main = do 
    input <- getContents
    putStrLn $ show (part1 (lines input))
    putStrLn $ show (part2 (lines input))
