import Data.List 

fours xs 
    | length xs <= 4 = [xs]
    | otherwise = (take 4 xs):(fours $ tail xs)

fourteens xs 
    | length xs <= 14 = [xs]
    | otherwise = (take 14 xs):(fourteens $ tail xs)

part1 s =
    let Just k = elemIndex 4 $ map (length . nub) (fours s)
    in  k+4

part2 s = 
    let Just k = elemIndex 14 $ map (length . nub) (fourteens s)
    in  k+14

main = do 
    input <- getContents
    putStrLn $ show (part1 input)
    putStrLn $ show (part2 input)
