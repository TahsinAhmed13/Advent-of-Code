import Data.List

contains1 (a, b, c, d) = if (a <= c && d <= b) || (c <= a && b <= d) then 1 else 0

contains2 (a, b, c, d) = if null (intersect [a..b] [c..d]) then 0 else 1

parse line = 
    let a = read $ takeWhile (/= '-') line :: Int
        b = read . takeWhile (/= ',') . tail $ dropWhile (/= '-') line :: Int
        c = read . takeWhile (/= '-') . tail $ dropWhile (/= ',') line :: Int
        d = read . tail . dropWhile (/= '-') . tail $ dropWhile (/= ',') line :: Int
    in  (a, b, c, d)

part1 lines = sum $ map (contains1 . parse) lines

part2 lines = sum $ map (contains2 . parse) lines

main = do 
    input <- getContents
    putStrLn $ show (part1 (lines input))
    putStrLn $ show (part2 (lines input))
