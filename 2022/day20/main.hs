import Data.List

parse input = 
    map read input :: [Int]

add i x ys = 
    let (first, second) = splitAt i ys
    in  first ++ x:second

remove i ys = 
    let (first, second) = splitAt i ys
    in  first ++ (tail second)

moveRight n x ys = 
    let (Just i) = elemIndex x ys
        j = (i+n) `mod` ((length ys)-1)
    in  add j x $ remove i ys
         
moveLeft n x ys = 
    reverse $ (moveRight n x $ reverse ys)

move (x, i) ys =
    if x >= 0 then moveRight (abs x) (x, i) ys
    else moveLeft (abs x) (x, i) ys

mix n xs =
    let ys = map (\i -> (xs !! i, i)) [0..(length xs)-1]
        ys' = foldl (\acc x -> move x acc) ys (concat $ replicate n ys)
    in  map fst ys'

part1 input = 
    let xs = mix 1 $ parse input
        (Just i) = elemIndex 0 xs
    in  sum $ map (\x -> xs !! ((i+x) `mod` (length xs))) [1000, 2000, 3000]

part2 input = 
    let xs = mix 10 . map (*811589153) $ parse input
        (Just i) = elemIndex 0 xs
    in  sum $ map (\x -> xs !! ((i+x) `mod` (length xs))) [1000, 2000, 3000]
        

main = do 
    input <- getContents 
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
