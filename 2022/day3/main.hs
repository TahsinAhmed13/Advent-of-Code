import Data.Char
import Data.List

priority c 
    | 'a' <= c && c <= 'z' = (ord c) - (ord 'a') + 1
    | otherwise = (ord c) - (ord 'A') + 27

process s =
    let size = (length s) `div` 2
        first = take size s
        second = drop size s
    in  priority . head $ intersect first second 

part1 xs = sum $ map process xs

part2 [] = 0
part2 (x:y:z:zs) =
    let badge = head (intersect x (intersect y z)) 
    in  priority badge + part2 zs

main = do 
    input <- getContents
    putStrLn $ show (part1 (lines input))
    putStrLn $ show (part2 (lines input))
