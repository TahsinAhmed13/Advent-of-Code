import Data.Char 

visible (i, j) grid = 
    let elem = (grid !! i) !! j
        ok xs = null $ filter (>=elem) xs
        left = take j (grid !! i)
        right = tail $ drop j (grid !! i)
        up = map (!!j) $ take i grid
        down = map (!!j) . tail $ drop i grid
    in  ok left || ok right || ok up || ok down

score (i, j) grid =
    let elem = (grid !! i) !! j
        left = reverse $ take j (grid !! i)
        right = tail $ drop j (grid !! i)
        up = reverse . map (!!j) $ take i grid
        down = map (!!j) . tail $ drop i grid
        count xs = min (length xs) . (+1) . length $ takeWhile (<elem) xs
    in  (count left) * (count right) * (count up) * (count down)

part1 input = 
    let trees = map (map (\x -> (ord x)-(ord '0'))) input
        width = length (input !! 0)
        height = length input
        coords = [(x, y) | x <- [0..(height-1)], y <- [0..(width-1)]]  
    in  length $ filter (flip visible $ trees) coords

part2 input =
    let trees = map (map (\x -> (ord x)-(ord '0'))) input
        width = length (input !! 0)
        height = length input
        coords = [(x, y) | x <- [0..(height-1)], y <- [0..(width-1)]]  
    in  maximum $ map (flip score $ trees) coords

main = do
    input <- getContents
    putStrLn $ show (part1 $ lines input)
    putStrLn $ show (part2 $ lines input)
